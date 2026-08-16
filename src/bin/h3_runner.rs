use std::env;
use std::fs;
use std::hint::black_box;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

const ARM_ID: &str = "H3-OS-NICE-CAUSAL-v0.1";
const DEFAULT_UNITS: u64 = 50_000_000;

fn arg(name: &str) -> Option<String> {
    let args: Vec<String> = env::args().collect();
    args.windows(2).find(|w| w[0] == name).map(|w| w[1].clone())
}

fn required(name: &str) -> String {
    arg(name).unwrap_or_else(|| panic!("missing required argument {name}"))
}

fn current_nice() -> i32 {
    let stat = fs::read_to_string("/proc/self/stat").expect("read /proc/self/stat");
    let close = stat.rfind(')').expect("parse /proc/self/stat");
    let fields: Vec<&str> = stat[close + 2..].split_whitespace().collect();
    fields[16].parse().expect("parse nice")
}

fn cpu_ticks() -> (u64, u64) {
    let stat = fs::read_to_string("/proc/self/stat").expect("read /proc/self/stat");
    let close = stat.rfind(')').expect("parse /proc/self/stat");
    let fields: Vec<&str> = stat[close + 2..].split_whitespace().collect();
    (fields[11].parse().unwrap(), fields[12].parse().unwrap())
}

fn workload(units: u64) -> u64 {
    let mut x = 0x9e37_79b9_7f4a_7c15u64;
    for i in 0..units {
        x = x.wrapping_add(i ^ 0xa5a5_a5a5_a5a5_a5a5);
        x ^= x.rotate_left(17);
        x = x.wrapping_mul(0x9e37_79b9_7f4a_7c15);
        if i & 0x1fff == 0 { black_box(x); }
    }
    black_box(x)
}

fn worker() -> ! {
    loop { workload(5_000_000); }
}

fn git_value(args: &[&str]) -> String {
    String::from_utf8(Command::new("git").args(args).output().expect("git").stdout)
        .unwrap_or_default().trim().to_string()
}

fn json_escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n")
}

fn main() {
    if env::args().any(|a| a == "--worker") { worker(); }

    if env::args().any(|a| a == "--validate-only") {
        assert_eq!(ARM_ID, "H3-OS-NICE-CAUSAL-v0.1");
        let nice: i32 = required("--expected-nice").parse().expect("expected nice");
        assert!(nice == 0 || nice == 10);
        println!("H3_RUNNER_VALIDATION=PASS");
        return;
    }

    let trial_id = required("--trial-id");
    let role = required("--role");
    let expected_nice: i32 = required("--expected-nice").parse().expect("expected nice");
    let environment_id = required("--environment-id");
    let workload_revision = required("--workload-revision");
    let arm_id = required("--arm-id");
    let units: u64 = arg("--work-units").unwrap_or_else(|| DEFAULT_UNITS.to_string()).parse().unwrap();

    assert_eq!(arm_id, ARM_ID, "unregistered execution arm");
    assert!(role == "control" || role == "treatment", "invalid role");
    assert!(expected_nice == 0 || expected_nice == 10, "unregistered niceness");

    let observed_nice = current_nice();
    assert_eq!(observed_nice, expected_nice, "actuator verification failed");

    // Force scheduler contention while keeping the registered target workload fixed.
    // Workers are explicitly reset to nice 0 so treatment receives lower priority.
    let worker_count = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
    let exe = env::current_exe().expect("current executable");
    let mut workers = Vec::with_capacity(worker_count);
    for _ in 0..worker_count {
        let child = Command::new("nice")
            .args(["-n", "0"])
            .arg(&exe)
            .arg("--worker")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("spawn contention worker");
        workers.push(child);
    }
    thread::sleep(Duration::from_millis(250));

    let (user_before, sys_before) = cpu_ticks();
    let start = Instant::now();
    let checksum = workload(units);
    let wall_ms = start.elapsed().as_secs_f64() * 1000.0;
    let (user_after, sys_after) = cpu_ticks();

    for mut worker in workers {
        let _ = worker.kill();
        let _ = worker.wait();
    }

    let repository = git_value(&["remote", "get-url", "origin"]);
    let commit = git_value(&["rev-parse", "HEAD"]);
    println!(
        "{{\"schema\":\"h3-trial-v0.1\",\"arm_id\":\"{}\",\"trial_id\":\"{}\",\"role\":\"{}\",\"expected_nice\":{},\"observed_nice\":{},\"environment_id\":\"{}\",\"repository\":\"{}\",\"commit\":\"{}\",\"workload_revision\":\"{}\",\"work_units\":{},\"wall_ms\":{:.3},\"user_ticks\":{},\"system_ticks\":{},\"checksum\":{}}}",
        ARM_ID,
        json_escape(&trial_id),
        role,
        expected_nice,
        observed_nice,
        json_escape(&environment_id),
        json_escape(&repository),
        commit,
        json_escape(&workload_revision),
        units,
        wall_ms,
        user_after - user_before,
        sys_after - sys_before,
        checksum,
    );
}
