use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

fn now_ms() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
}

fn current_cgroup_path() -> Option<String> {
    let text = fs::read_to_string("/proc/self/cgroup").ok()?;
    text.lines().find_map(|line| {
        let mut p = line.splitn(3, ':');
        let hierarchy = p.next()?;
        let _controllers = p.next()?;
        let path = p.next()?;
        if hierarchy == "0" {
            Some(path.to_string())
        } else {
            None
        }
    })
}

fn root() -> PathBuf {
    PathBuf::from("/sys/fs/cgroup")
}

fn main() -> io::Result<()> {
    let root = root();
    let controllers = fs::read_to_string(root.join("cgroup.controllers")).unwrap_or_default();
    let controllers_vec = controllers
        .split_whitespace()
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let v2 = root.join("cgroup.controllers").is_file();
    let cpu_available = controllers_vec.iter().any(|c| c == "cpu");
    let current = current_cgroup_path();
    let parent = current
        .as_deref()
        .map(|p| root.join(p.trim_start_matches('/')));
    let cpu_max = parent
        .as_ref()
        .and_then(|p| fs::read_to_string(p.join("cpu.max")).ok())
        .map(|s| s.trim().to_string());

    let mut child_created = false;
    let mut child_cpu_max_before = None;
    let mut cpu_max_writable = false;
    let mut cpu_max_write_error = None;
    let mut child_cpu_max_after = None;
    let mut child_migration = false;
    let mut child_observed_cpu_max = None;
    let mut child_observation_matches = false;
    let mut cleanup_ok = false;
    let mut reason = None;
    const EXPECTED_CPU_MAX: &str = "10000 100000";

    if !v2 {
        reason = Some("cgroup v2 unavailable".to_string());
    } else if !cpu_available {
        reason = Some("cpu controller unavailable".to_string());
    } else if let Some(parent) = parent {
        let name = format!("a2_probe_{}_{}", std::process::id(), now_ms());
        let child = parent.join(&name);
        match fs::create_dir(&child) {
            Ok(()) => {
                child_created = true;
                let max_path = child.join("cpu.max");
                child_cpu_max_before = fs::read_to_string(&max_path)
                    .ok()
                    .map(|s| s.trim().to_string());

                match fs::OpenOptions::new().write(true).open(&max_path) {
                    Ok(mut f) => {
                        cpu_max_writable = f.write_all(EXPECTED_CPU_MAX.as_bytes()).is_ok();
                        if !cpu_max_writable {
                            cpu_max_write_error = Some("write_all failed".to_string());
                        }
                    }
                    Err(e) => {
                        cpu_max_write_error = Some(e.to_string());
                    }
                }

                child_cpu_max_after = fs::read_to_string(&max_path)
                    .ok()
                    .map(|s| s.trim().to_string());

                let mut probe = Command::new("sh")
                    .arg("-c")
                    .arg("printf '%s\\n' \"$$\" > /tmp/a2_probe_child_pid; sleep 5")
                    .stdout(Stdio::null())
                    .spawn()?;
                let pid = probe.id();

                child_migration = fs::write(child.join("cgroup.procs"), pid.to_string()).is_ok();
                if child_migration {
                    let proc_cgroup = fs::read_to_string(format!("/proc/{pid}/cgroup"))
                        .unwrap_or_default();
                    let migrated_path = proc_cgroup
                        .lines()
                        .find_map(|line| line.strip_prefix("0::"));
                    let expected_child = child
                        .strip_prefix(&root)
                        .ok()
                        .map(|p| format!("/{}", p.display()))
                        .unwrap_or_default();
                    child_migration = migrated_path == Some(expected_child.as_str());

                    if child_migration {
                        child_observed_cpu_max = fs::read_to_string(&max_path)
                            .ok()
                            .map(|s| s.trim().to_string());
                        child_observation_matches = child_observed_cpu_max.as_deref()
                            == Some(EXPECTED_CPU_MAX);
                    }
                }

                let _ = probe.wait();
                cleanup_ok = fs::remove_dir(&child).is_ok();
            }
            Err(e) => reason = Some(format!("cannot create child cgroup: {e}")),
        }
    } else {
        reason = Some("cannot determine current cgroup path".to_string());
    }

    let capable = v2
        && cpu_available
        && cpu_max.is_some()
        && child_created
        && cpu_max_writable
        && child_cpu_max_after.as_deref() == Some(EXPECTED_CPU_MAX)
        && child_migration
        && child_observation_matches
        && cleanup_ok;

    if !capable && reason.is_none() {
        reason = Some(format!(
            "required capability missing: child_created={child_created}, cpu_max_writable={cpu_max_writable}, child_migration={child_migration}, child_observation_matches={child_observation_matches}, cleanup_ok={cleanup_ok}"
        ));
    }

    println!("A2_CAPABILITY_REPORT");
    println!("timestamp_ms={}", now_ms());
    println!("cgroup_v2={v2}");
    println!("mount={}", root.display());
    println!("current_cgroup={}", current.unwrap_or_default());
    println!("controllers={}", controllers_vec.join(","));
    println!("cpu_available={cpu_available}");
    println!("cpu_max={}", cpu_max.unwrap_or_default());
    println!("expected_cpu_max={EXPECTED_CPU_MAX}");
    println!("child_created={child_created}");
    println!("child_cpu_max_before={}", child_cpu_max_before.unwrap_or_default());
    println!("cpu_max_writable={cpu_max_writable}");
    println!("cpu_max_write_error={}", cpu_max_write_error.unwrap_or_default());
    println!("child_cpu_max_after={}", child_cpu_max_after.unwrap_or_default());
    println!("child_migration={child_migration}");
    println!("child_observed_cpu_max={}", child_observed_cpu_max.unwrap_or_default());
    println!("child_observation_matches={child_observation_matches}");
    println!("cleanup_ok={cleanup_ok}");
    println!("capable={capable}");
    println!("reason={}", reason.unwrap_or_default());
    Ok(())
}
