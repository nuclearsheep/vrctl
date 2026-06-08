use sysinfo::{System, Pid};

pub fn run_process_tree_scan(sys: &System) {
    println!("\n--- [ 🌳 Process Tree Scan ] ---");
    
    let wivrn_pids = find_pids_by_name(sys, "wivrn");
    if wivrn_pids.is_empty() {
        println!("[❌] WiVRn Server: NOT RUNNING");
    } else {
        for pid in &wivrn_pids {
            println!("[🟩] WiVRn Server Found! (PID: {})", pid);
            print_child_processes(sys, *pid, 1);
        }
    }

    let vrchat_pids = find_pids_by_name(sys, "VRChat");
    if vrchat_pids.is_empty() {
        println!("[❓] VRChat Game: Not Detected [cite: 31]");
    } else {
        for pid in &vrchat_pids {
            println!("[🎮] VRChat Process Active! (PID: {})", pid);
            println!("    └── Ancestry Chain:");
            trace_parent_chain(sys, *pid);
        }
    }
}

fn find_pids_by_name(sys: &System, name: &str) -> Vec<sysinfo::Pid> {
    sys.processes()
        .iter()
        .filter(|(_, proc)| proc.name().to_string().to_lowercase().contains(&name.to_lowercase()))
        .map(|(pid, _)| *pid)
        .collect()
}

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
