use std::env;
use std::fs;
use std::path::Path;
use sysinfo::{Pid, System};

/// Core entry point for our environment scan command
pub fn run_environment_scan(sys: &System) {
    println!("\n--- [ 🔍 Environment Scan ] ---");

    // 1. Check Host Level OS Variable
    match env::var("XR_RUNTIME_JSON") {
        Ok(val) => {
            println!("[🟩] Host Environment: XR_RUNTIME_JSON is set.");
            println!("    └── Target Path: {}", val);
            
            // Validate if the file physically exists on disk
            if Path::new(&val).exists() {
                println!("    └── Path Status: VALID (File exists) ");
            } else {
                println!("    └── [❌] Path Status: INVALID (File does not exist or is missing client trigger!) [cite: 52, 314]");
            }
        }
        Err(_) => {
            println!("[❌] Host Environment: XR_RUNTIME_JSON is NOT set in your active terminal session. [cite: 281]");
        }
    }

    // 2. Scan for VRChat inside Proton/Pressure-Vessel Sandbox
    let mut found_vrchat = false;
    for (pid, proc) in sys.processes() {
        if proc.name().to_string().to_lowercase().contains("vrchat") {
            found_vrchat = true;
            println!("[🎮] Active VRChat Process Found (PID: {})", pid);
            inspect_proton_environment(*pid);
        }
    }

    if !found_vrchat {
        println!("[❓] Proton Context: VRChat is not currently running; cannot inspect sandbox variables. [cite: 31]");
    }
}

/// Scans the procfs memory tracking paths on Linux to leak Proton's environment variables
fn inspect_proton_environment(pid: Pid) {
    // Linux exposes the raw environment variables block handed to any process via /proc/<pid>/environ
    let environ_path = format!("/proc/{}/environ", pid);
    
    match fs::read_to_string(&environ_path) {
        Ok(content) => {
            // Procfs splits environment variables using null bytes ('\0') instead of newlines
            let mut xr_runtime_found = false;
            let mut vr_override_found = false;

            for var in content.split('\0') {
                if var.starts_with("XR_RUNTIME_JSON=") {
                    println!("    ├── [Sandbox Variable] {}", var);
                    xr_runtime_found = true;
                }
                if var.starts_with("VR_OVERRIDE=") {
                    println!("    ├── [Sandbox Variable] {}", var);
                    vr_override_found = true;
                }
            }

            if !xr_runtime_found {
                println!("    ├── [❌] ERROR: XR_RUNTIME_JSON is missing from the Proton sandbox! ");
            }
            if !vr_override_found {
                println!("    └── [⚠️] WARNING: VR_OVERRIDE is missing from the Proton sandbox! [cite: 28, 47]");
            }
        }
        Err(_) => {
            println!("    └── [❌] Access Denied: Run vrctl with 'sudo' to inspect the Proton container parameters. ");
        }
    }
}
