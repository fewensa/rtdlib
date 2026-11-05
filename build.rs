// Warn when this process cannot see enough RAM to compile the generated types.
// A 1 GB limit kills rustc with signal 9. The check is a warning so a
// mis-read cgroup does not fail a build that would have succeeded.

use std::fs;
use std::path::Path;

/// Dev-build peak RSS of this crate on rustc 1.89 (`cargo build -j 1`).
const DEV_BUILD_PEAK_BYTES: u64 = 6 * 1024 * 1024 * 1024;

fn main() {
  println!("cargo:rerun-if-changed=build.rs");
  if let Some(limit) = memory_limit_bytes() {
    if limit < DEV_BUILD_PEAK_BYTES {
      println!(
        "cargo:warning=Compiling rtdlib needs about 6 GB of RAM for one rustc process. This environment exposes {}. rustc can be killed with signal 9 (out of memory). See the Building section of README.md.",
        format_gib(limit)
      );
    }
  }
}

fn memory_limit_bytes() -> Option<u64> {
  let mut limit = mem_total_bytes();
  if let Some(cgroup) = cgroup_memory_max_bytes() {
    limit = Some(match limit {
      Some(total) => total.min(cgroup),
      None => cgroup,
    });
  }
  limit
}

fn mem_total_bytes() -> Option<u64> {
  let text = fs::read_to_string("/proc/meminfo").ok()?;
  for line in text.lines() {
    let Some(rest) = line.strip_prefix("MemTotal:") else {
      continue;
    };
    let kib: u64 = rest.split_whitespace().next()?.parse().ok()?;
    return Some(kib.saturating_mul(1024));
  }
  None
}

fn cgroup_memory_max_bytes() -> Option<u64> {
  let text = fs::read_to_string("/proc/self/cgroup").ok()?;
  let mut best: Option<u64> = None;
  for line in text.lines() {
    if let Some(rel) = line.strip_prefix("0::") {
      consider(&mut best, v2_limit(rel.trim_start_matches('/')));
      continue;
    }
    // cgroup v1: "12:memory:/docker/<id>"
    let mut parts = line.splitn(3, ':');
    let _id = parts.next();
    let controllers = parts.next().unwrap_or("");
    let rel = parts.next().unwrap_or("");
    if controllers.split(',').any(|c| c == "memory") {
      consider(&mut best, v1_limit(rel.trim_start_matches('/')));
    }
  }
  best
}

fn consider(best: &mut Option<u64>, value: Option<u64>) {
  if let Some(value) = value {
    *best = Some(best.map_or(value, |prev| prev.min(value)));
  }
}

fn v2_limit(relative: &str) -> Option<u64> {
  let mut path = Path::new("/sys/fs/cgroup").join(relative);
  let mut best: Option<u64> = None;
  loop {
    if let Some(bytes) = read_max(&path.join("memory.max")) {
      best = Some(best.map_or(bytes, |prev| prev.min(bytes)));
    }
    if path.as_os_str() == "/sys/fs/cgroup" || !path.pop() {
      break;
    }
  }
  best
}

fn v1_limit(relative: &str) -> Option<u64> {
  let mut path = Path::new("/sys/fs/cgroup/memory").join(relative);
  let mut best: Option<u64> = None;
  loop {
    if let Some(bytes) = read_max(&path.join("memory.limit_in_bytes")) {
      best = Some(best.map_or(bytes, |prev| prev.min(bytes)));
    }
    if path.as_os_str() == "/sys/fs/cgroup/memory" || !path.pop() {
      break;
    }
  }
  best
}

/// `memory.max` uses the word `max` when the controller is unlimited.
/// cgroup v1 uses a value near `i64::MAX` for the same case.
fn read_max(path: &Path) -> Option<u64> {
  let text = fs::read_to_string(path).ok()?;
  let text = text.trim();
  if text == "max" {
    return None;
  }
  let bytes: u64 = text.parse().ok()?;
  if bytes > (1_u64 << 62) {
    return None;
  }
  Some(bytes)
}

fn format_gib(bytes: u64) -> String {
  let gib = bytes as f64 / (1024.0 * 1024.0 * 1024.0);
  format!("{gib:.1} GiB")
}
