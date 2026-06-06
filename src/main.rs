//fn main() {
//    println!("Hello, world!");
//}
//

use std::thread;
use std::time::Duration;
use sysinfo::{Process, System};

fn main() {
    // Instantiate the system info struct once and fill it with all process data
    let mut sys = System::new_all();

    println!("==================================================");
    println!("  vrctl: Headless XR Diagnostic Sniffer Initialized");
    println!("==================================================");

    loop {
        // Refresh the process list on every iteration of our monitoring loop
        sys.refresh_all();

        println!("\n--- [ System Scan ] ---");

        // 1. Check for the WiVRn Background Server
        let wivrn_pids = find_pids_by_name(&sys, "wivrn");
        if wivrn_pids.is_empty() {
            println!("[❌] WiVRn Server: NOT RUNNING");
        } else {
            for pid in &wivrn_pids {
                println!("[🟩] WiVRn Server Found! (PID: {})", pid);
                // Print any child processes attached to the WiVRn server itself
                print_child_processes(&sys, *pid, 1);
            }
        }

        // 2. Check for VRChat or Proton/Wine Containers
        let vrchat_pids = find_pids_by_name(&sys, "VRChat");
        if vrchat_pids.is_empty() {
            println!("[❓] VRChat Game: Not Detected");
        } else {
            for pid in &vrchat_pids {
                println!("[🎮] VRChat Process Active! (PID: {})", pid);
                
                // Trace up the family tree to see who launched VRChat (Steam, Terminal, Script?)
                println!("    └── Ancestry Chain:");
                trace_parent_chain(&sys, *pid);
            }
        }

        // 3. Keep an eye out for other active runtimes/compositors
        let wayvr_pids = find_pids_by_name(&sys, "wayvr");
        if !wayvr_pids.is_empty() {
            println!("[🖥️] WayVR Compositor: ACTIVE (PIDs: {:?})", wayvr_pids);
        }

        // Sleep for 3 seconds so we don't obliterate CPU usage while polling
        thread::sleep(Duration::from_secs(3));
    }
}

/// Helper function to search all active processes for a specific string match (case-insensitive)
fn find_pids_by_name(sys: &System, name: &str) -> Vec<sysinfo::Pid> {
    sys.processes()
        .iter()
        .filter(|(_, proc)| proc.name().to_string().to_lowercase().contains(&name.to_lowercase()))
        .map(|(pid, _)| *pid)
        .collect()
}

/// Recursively prints any child processes spawned by a specific Parent PID
fn print_child_processes(sys: &System, parent_pid: sysinfo::Pid, depth: usize) {
    let indent = "    ".repeat(depth);
    for (pid, proc) in sys.processes() {
        if let Some(ppid) = proc.parent() {
            if ppid == parent_pid {
                println!("{}└── Child: {} (PID: {})", indent, proc.name(), pid);
                print_child_processes(sys, *pid, depth + 1);
            }
        }
    }
}

/// Recursively traces upwards from a given PID to print its parent processes
fn trace_parent_chain(sys: &System, current_pid: sysinfo::Pid) {
    if let Some(proc) = sys.process(current_pid) {
        if let Some(parent_pid) = proc.parent() {
            if let Some(parent_proc) = sys.process(parent_pid) {
                println!("        └── Spawned by: {} (PID: {})", parent_proc.name(), parent_pid);
                trace_parent_chain(sys, parent_pid);
            }
        }
    }
}
