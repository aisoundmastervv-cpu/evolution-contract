use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
struct Report {
    timestamp_ms: u128,
    cgroup_v2: bool,
    mount: Option<String>,
    current_cgroup: Option<String>,
    controllers: Vec<String>,
    cpu_available: bool,
    cpu_max: Option<String>,
    child_created: bool,
    cpu_max_writable: bool,
    child_migration: bool,
    child_observed_cpu_max: Option<String>,
    cleanup_ok: bool,
    capable: bool,
    reason: Option<String>,
}

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
        if hierarchy.is_empty() { Some(path.to_string()) } else { None }
    })
}

fn cgroup_root() -> PathBuf { PathBuf::from("/sys/fs/cgroup") }

fn main() -> io::Result<()> {
    let root = cgroup_root();
    let controllers_path = root.join("cgroup.controllers");
    let cgroup_v2 = controllers_path.is_file();
    let current = current_cgroup_path();
    let current_dir = current.as_deref().map(|p| root.join(p.trim_start_matches('/')));

    let controllers = fs::read_to_string(&controllers_path)
        .unwrap_or_default()
        .split_whitespace().map(str::to_owned).collect::<Vec<_>>();
    let cpu_available = controllers.iter().any(|c| c == "cpu");
    let cpu_max = current_dir.as_ref()
        .and_then(|d| fs::read_to_string(d.join("cpu.max")).ok())
        .map(|s| s.trim().to_string());

    let mut child_created = false;
    let mut cpu_max_writable = false;
    let mut child_migration = false;
    let mut child_observed_cpu_max = None;
    let mut cleanup_ok = false;
    let mut reason = None;

    if !cgroup_v2 { reason = Some("cgroup v2 unavailable".into()); }
    else if !cpu_available { reason = Some("cpu controller unavailable in current cgroup hierarchy".into()); }
    else if let Some(parent) = current_dir {
        let name = format!("a2_probe_{}_{}", std::process::id(), now_ms());
        let child = parent.join(&name);
        match fs::create_dir(&child) {
            Ok(()) => {
                child_created = true;
                let max_path = child.join("cpu.max");
                match fs::OpenOptions::new().write(true).open(&max_path) {
                    Ok(mut f) => {
                        cpu_max_writable = f.write_all(b"max 100000").is_ok();
                    }
                    Err(_) => {}
                }

                // Use a separate short-lived child so the harness itself never enters
                // the intervention cgroup. The child prints its effective cpu.max.
                let mut child_proc = Command::new("sh")
                    .arg("-c")
                    .arg("cat /proc/self/cgroup; cat /sys/fs/cgroup/$(awk -F: 'BEGIN{OFS=\"\"} $1==\"\"{print $3}' /proc/self/cgroup)/cpu.max 2>/dev/null || true")
                    .stdout(Stdio::piped())
                    .spawn()?;

                let pid = child_proc.id();
                child_migration = fs::write(child.join("cgroup.procs"), pid.to_string()).is_ok();
                let output = child_proc.wait_with_output()?;
                let stdout = String::from_utf8_lossy(&output.stdout);
                child_observed_cpu_max = stdout.lines().rev().find(|l| l.contains(' ')).map(str::to_owned)
                    .or_else(|| fs::read_to_string(&max_path).ok().map(|s| s.trim().to_string()));

                cleanup_ok = fs::remove_dir(&child).is_ok();
            }
            Err(e) => reason = Some(format!("cannot create child cgroup: {e}")),
        }
    } else {
        reason = Some("cannot determine current cgroup path".into());
    }

    let capable = cgroup_v2 && cpu_available && cpu_max.is_some()
        && child_created && cpu_max_writable && child_migration && cleanup_ok;
    if !capable && reason.is_none() {
        reason = Some(format!(
            "required capability missing: child_created={child_created}, cpu_max_writable={cpu_max_writable}, child_migration={child_migration}, cleanup_ok={cleanup_ok}"
        ));
    }

    let report = Report { timestamp_ms: now_ms(), cgroup_v2, mount: Some(root.display().to_string()),
        current_cgroup: current, controllers, cpu_available, cpu_max, child_created,
        cpu_max_writable, child_migration, child_observed_cpu_max, cleanup_ok, capable, reason };

    println!("A2_CAPABILITY_REPORT");
    println!("timestamp_ms={}", report.timestamp_ms);
    println!("cgroup_v2={}", report.cgroup_v2);
    println!("mount={}", report.mount.unwrap_or_default());
    println!("current_cgroup={}", report.current_cgroup.unwrap_or_default());
    println!("controllers={}", report.controllers.join(","));
    println!("cpu_available={}", report.cpu_available);
    println!("cpu_max={}", report.cpu_max.unwrap_or_default());
    println!("child_created={}", report.child_created);
    println!("cpu_max_writable={}", report.cpu_max_writable);
    println!("child_migration={}", report.child_migration);
    println!("child_observed_cpu_max={}", report.child_observed_cpu_max.unwrap_or_default());
    println!("cleanup_ok={}", report.cleanup_ok);
    println!("capable={}", report.capable);
    println!("reason={}", report.reason.unwrap_or_default());

    // Capability discovery never emits a semantic experiment verdict.
    // Exit 0 means the probe completed; capable=true/false is evidence only.
    Ok(())
}
