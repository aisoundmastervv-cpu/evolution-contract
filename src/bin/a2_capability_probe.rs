use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

fn now_ms() -> u128 { SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() }
fn current_cgroup_path() -> Option<String> {
    let text = fs::read_to_string("/proc/self/cgroup").ok()?;
    text.lines().find_map(|line| {
        let mut p = line.splitn(3, ':');
        let hierarchy = p.next()?;
        let _ = p.next()?;
        let path = p.next()?;
        if hierarchy.is_empty() { Some(path.to_string()) } else { None }
    })
}
fn root() -> PathBuf { PathBuf::from("/sys/fs/cgroup") }

fn main() -> io::Result<()> {
    let root = root();
    let controllers = fs::read_to_string(root.join("cgroup.controllers")).unwrap_or_default();
    let controllers_vec = controllers.split_whitespace().map(str::to_owned).collect::<Vec<_>>();
    let v2 = root.join("cgroup.controllers").is_file();
    let cpu_available = controllers_vec.iter().any(|c| c == "cpu");
    let current = current_cgroup_path();
    let parent = current.as_deref().map(|p| root.join(p.trim_start_matches('/')));
    let cpu_max = parent.as_ref().and_then(|p| fs::read_to_string(p.join("cpu.max")).ok()).map(|s| s.trim().to_string());

    let mut child_created = false;
    let mut cpu_max_writable = false;
    let mut child_migration = false;
    let mut child_observed_cpu_max = None;
    let mut cleanup_ok = false;
    let mut reason = None;

    if !v2 { reason = Some("cgroup v2 unavailable".to_string()); }
    else if !cpu_available { reason = Some("cpu controller unavailable".to_string()); }
    else if let Some(parent) = parent {
        let name = format!("a2_probe_{}_{}", std::process::id(), now_ms());
        let child = parent.join(&name);
        match fs::create_dir(&child) {
            Ok(()) => {
                child_created = true;
                let max_path = child.join("cpu.max");
                if let Ok(mut f) = fs::OpenOptions::new().write(true).open(&max_path) {
                    cpu_max_writable = f.write_all(b"max 100000").is_ok();
                }

                // Keep the probe process alive while its PID is migrated. The harness
                // itself never enters the intervention cgroup.
                let mut probe = Command::new("sh")
                    .arg("-c")
                    .arg("echo $$ > /tmp/a2_probe_pid; sleep 5")
                    .stdout(Stdio::null())
                    .spawn()?;
                let pid = probe.id();
                child_migration = fs::write(child.join("cgroup.procs"), pid.to_string()).is_ok();

                // Verify migration while the process is still alive.
                if child_migration {
                    let proc_cgroup = fs::read_to_string(format!("/proc/{pid}/cgroup")).unwrap_or_default();
                    let expected = format!("0::{}", current.as_deref().unwrap_or(""));
                    let migrated_path = proc_cgroup.lines().find_map(|line| line.strip_prefix("0::"));
                    let expected_child = child.strip_prefix(&root).ok().map(|p| format!("/{}", p.display())).unwrap_or_default();
                    child_migration = migrated_path == Some(expected_child.as_str()) || proc_cgroup.contains(&expected);
                }

                let _ = probe.wait();
                child_observed_cpu_max = fs::read_to_string(&max_path).ok().map(|s| s.trim().to_string());
                cleanup_ok = fs::remove_dir(&child).is_ok();
            }
            Err(e) => reason = Some(format!("cannot create child cgroup: {e}")),
        }
    } else { reason = Some("cannot determine current cgroup path".to_string()); }

    let capable = v2 && cpu_available && cpu_max.is_some() && child_created && cpu_max_writable && child_migration && cleanup_ok;
    if !capable && reason.is_none() {
        reason = Some(format!("required capability missing: child_created={child_created}, cpu_max_writable={cpu_max_writable}, child_migration={child_migration}, cleanup_ok={cleanup_ok}"));
    }

    println!("A2_CAPABILITY_REPORT");
    println!("timestamp_ms={}", now_ms());
    println!("cgroup_v2={v2}");
    println!("mount={}", root.display());
    println!("current_cgroup={}", current.unwrap_or_default());
    println!("controllers={}", controllers_vec.join(","));
    println!("cpu_available={cpu_available}");
    println!("cpu_max={}", cpu_max.unwrap_or_default());
    println!("child_created={child_created}");
    println!("cpu_max_writable={cpu_max_writable}");
    println!("child_migration={child_migration}");
    println!("child_observed_cpu_max={}", child_observed_cpu_max.unwrap_or_default());
    println!("cleanup_ok={cleanup_ok}");
    println!("capable={capable}");
    println!("reason={}", reason.unwrap_or_default());
    // Exit status describes probe execution only, never a semantic A2 verdict.
    Ok(())
}
