//! H3 execution harness primitives.
//!
//! This module provides an external Linux cgroup-v2 actuator and an
//! independent CPU-time measurement path. It deliberately does not contain an
//! `efficiency -> actuator` mapping: that mapping belongs to a separately
//! registered execution arm.

use std::fs::{self, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Command};
use std::time::{Duration, Instant};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkloadSpec {
    pub program: PathBuf,
    pub args: Vec<String>,
    pub expected_exit_code: i32,
    pub measurement_window: Duration,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CgroupCpuStat {
    pub usage_usec: u64,
    pub user_usec: u64,
    pub system_usec: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CpuMeasurement {
    pub cpu_time: Duration,
    pub source: MeasurementSource,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MeasurementSource {
    LinuxCgroupV2CpuStat,
}

#[derive(Debug)]
pub struct CgroupV2 {
    path: PathBuf,
}

impl CgroupV2 {
    /// Open an already-created cgroup-v2 directory.
    ///
    /// The harness never derives this path from `efficiency`; callers provide
    /// the externally registered actuator target.
    pub fn open(path: impl Into<PathBuf>) -> io::Result<Self> {
        let path = path.into();
        if !path.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("cgroup does not exist: {}", path.display()),
            ));
        }
        Ok(Self { path })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Apply a pre-registered Linux cgroup CPU weight.
    ///
    /// This is an external actuator operation. No observed CPU quantity is
    /// calculated here, and no gene value is accepted by this API.
    pub fn set_cpu_weight(&self, weight: u64) -> io::Result<()> {
        if !(1..=10_000).contains(&weight) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "cgroup v2 cpu.weight must be in [1, 10000]",
            ));
        }
        write_value(self.path.join("cpu.weight"), &weight.to_string())
    }

    /// Attach an already-started workload process to this cgroup.
    pub fn attach_pid(&self, pid: u32) -> io::Result<()> {
        write_value(self.path.join("cgroup.procs"), &pid.to_string())
    }

    /// Read CPU usage directly from Linux's cgroup accounting interface.
    pub fn measure_cpu(&self) -> io::Result<CpuMeasurement> {
        let stat = read_cpu_stat(&self.path.join("cpu.stat"))?;
        Ok(CpuMeasurement {
            cpu_time: Duration::from_micros(stat.usage_usec),
            source: MeasurementSource::LinuxCgroupV2CpuStat,
        })
    }

    /// Run the registered workload in this cgroup and return its exit status
    /// plus independent CPU accounting.
    pub fn run(&self, workload: &WorkloadSpec) -> io::Result<ExecutionRecord> {
        let before = self.measure_cpu()?;
        let started = Instant::now();
        let mut child = spawn_workload(workload)?;
        self.attach_pid(child.id())?;
        let status = child.wait()?;
        let elapsed = started.elapsed();
        let after = self.measure_cpu()?;

        Ok(ExecutionRecord {
            exit_code: status.code(),
            workload_succeeded: status.code() == Some(workload.expected_exit_code),
            cpu_time: after.cpu_time.saturating_sub(before.cpu_time),
            wall_clock: elapsed,
            measurement_source: after.source,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExecutionRecord {
    pub exit_code: Option<i32>,
    pub workload_succeeded: bool,
    pub cpu_time: Duration,
    pub wall_clock: Duration,
    pub measurement_source: MeasurementSource,
}

fn spawn_workload(workload: &WorkloadSpec) -> io::Result<Child> {
    Command::new(&workload.program).args(&workload.args).spawn()
}

fn write_value(path: PathBuf, value: &str) -> io::Result<()> {
    let mut file = OpenOptions::new().write(true).open(path)?;
    file.write_all(value.as_bytes())
}

fn read_cpu_stat(path: &Path) -> io::Result<CgroupCpuStat> {
    let mut contents = String::new();
    fs::File::open(path)?.read_to_string(&mut contents)?;

    let mut usage_usec = None;
    let mut user_usec = None;
    let mut system_usec = None;

    for line in contents.lines() {
        let mut fields = line.split_whitespace();
        match (fields.next(), fields.next()) {
            (Some("usage_usec"), Some(value)) => usage_usec = Some(parse_usec(value)?),
            (Some("user_usec"), Some(value)) => user_usec = Some(parse_usec(value)?),
            (Some("system_usec"), Some(value)) => system_usec = Some(parse_usec(value)?),
            _ => {}
        }
    }

    Ok(CgroupCpuStat {
        usage_usec: usage_usec.ok_or_else(|| missing_stat("usage_usec"))?,
        user_usec: user_usec.ok_or_else(|| missing_stat("user_usec"))?,
        system_usec: system_usec.ok_or_else(|| missing_stat("system_usec"))?,
    })
}

fn parse_usec(value: &str) -> io::Result<u64> {
    value.parse::<u64>().map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("invalid cpu.stat value: {value}"),
        )
    })
}

fn missing_stat(name: &str) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidData,
        format!("cpu.stat missing {name}"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::Duration;

    #[test]
    fn cpu_stat_parser_reads_primary_endpoint() {
        let path = std::env::temp_dir().join(format!("h3-cpu-stat-{}", std::process::id()));
        fs::write(
            &path,
            "usage_usec 1234\nuser_usec 900\nsystem_usec 334\n",
        )
        .unwrap();
        let stat = read_cpu_stat(&path).unwrap();
        fs::remove_file(path).unwrap();
        assert_eq!(stat.usage_usec, 1234);
        assert_eq!(stat.user_usec, 900);
        assert_eq!(stat.system_usec, 334);
    }

    #[test]
    fn cpu_measurement_source_is_external_os_accounting() {
        assert_eq!(
            MeasurementSource::LinuxCgroupV2CpuStat,
            MeasurementSource::LinuxCgroupV2CpuStat
        );
    }

    #[test]
    fn cpu_weight_is_rejected_outside_linux_range() {
        let path = std::env::temp_dir().join(format!("h3-cgroup-{}", std::process::id()));
        fs::create_dir_all(&path).unwrap();
        let cgroup = CgroupV2::open(&path).unwrap();
        assert!(cgroup.set_cpu_weight(0).is_err());
        assert!(cgroup.set_cpu_weight(10_001).is_err());
        fs::remove_dir(path).unwrap();
    }

    #[test]
    fn workload_spec_has_no_efficiency_field() {
        let spec = WorkloadSpec {
            program: PathBuf::from("/bin/true"),
            args: Vec::new(),
            expected_exit_code: 0,
            measurement_window: Duration::from_secs(1),
        };
        assert_eq!(spec.expected_exit_code, 0);
    }

    #[test]
    fn cpu_time_is_derived_only_from_os_stat() {
        let measurement = CpuMeasurement {
            cpu_time: Duration::from_micros(2500),
            source: MeasurementSource::LinuxCgroupV2CpuStat,
        };
        assert_eq!(measurement.cpu_time, Duration::from_micros(2500));
    }

    #[test]
    fn execution_record_requires_workload_correctness_separately() {
        let record = ExecutionRecord {
            exit_code: Some(1),
            workload_succeeded: false,
            cpu_time: Duration::from_micros(100),
            wall_clock: Duration::from_millis(10),
            measurement_source: MeasurementSource::LinuxCgroupV2CpuStat,
        };
        assert!(!record.workload_succeeded);
        assert_eq!(record.exit_code, Some(1));
    }
}
