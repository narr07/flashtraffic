use std::sync::Mutex;
use std::process::Command;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

// Windows CREATE_NO_WINDOW flag to prevent console flickering
#[cfg(windows)]
pub const CREATE_NO_WINDOW: u32 = 0x0800_0000;

static ACTIVE_PIDS: Mutex<Vec<u32>> = Mutex::new(Vec::new());

/// Registers a child process PID for lifecycle tracking
pub fn register_pid(pid: u32) {
    if let Ok(mut pids) = ACTIVE_PIDS.lock() {
        pids.push(pid);
    }
}

/// Unregisters a completed child process PID
pub fn unregister_pid(pid: u32) {
    if let Ok(mut pids) = ACTIVE_PIDS.lock() {
        pids.retain(|&p| p != pid);
    }
}

/// Kills all registered child processes cleanly (anti-orphan guard)
pub fn kill_all_child_processes() {
    if let Ok(mut pids) = ACTIVE_PIDS.lock() {
        for pid in pids.drain(..) {
            #[cfg(windows)]
            {
                let mut cmd = Command::new("taskkill");
                cmd.creation_flags(CREATE_NO_WINDOW);
                cmd.args(["/F", "/T", "/PID", &pid.to_string()]);
                let _ = cmd.output();
            }
            #[cfg(not(windows))]
            {
                let _ = Command::new("kill").args(["-9", &pid.to_string()]).output();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pid_registration() {
        register_pid(999999);
        {
            let pids = ACTIVE_PIDS.lock().unwrap();
            assert!(pids.contains(&999999));
        }
        unregister_pid(999999);
        {
            let pids = ACTIVE_PIDS.lock().unwrap();
            assert!(!pids.contains(&999999));
        }
    }
}
