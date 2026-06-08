use std::thread;
use std::time::Duration;
use sysinfo::System;

// Declare our custom separate modules
mod process;
mod scanner;

fn main() {
    let mut sys = System::new_all();

    println!("==================================================");
    println!("  vrctl: Headless XR Multi-Sense Diagnostic Tool");
    println!("==================================================");

    loop {
        // Hydrate our process and hardware maps
        sys.refresh_all();

        // Sense 1: Check the process dependencies tree
        process::run_process_tree_scan(&sys);

        // Sense 2: Penetrate sandboxes to verify target path inheritance
        scanner::run_environment_scan(&sys);

        println!("\n==================================================");
        thread::sleep(Duration::from_secs(4));
    }
}
