use std::fs;
use std::path::Path;
use sysinfo::{System, Pid};

pub fn run_xrizer_diagnostic(sys: &System) {
    println!("\n--- [ 🫱🏼‍🫲🏼 Xrizer Bridge Diagnostic ] ---");

    // 1. Hardcoded target path based on your launch options configuration
    let xrizer_dir = "/home/alezarine/xrizer/target/release";
    let path = Path::new(xrizer_dir);

    if !path.exists() {
        println!("[❌] Path Status: Target path does not exist!");
        println!("    └── Expected: {}", xrizer_dir);
        return;
    }

    // Check for the compiled dynamic library files that xrizer produces
    let compiled_so = path.join("libxrizer.so"); 
    let openvr_so = path.join("libopenvr_api.so"); // Depending on version, it mimics this name

    if compiled_so.exists() || openvr_so.exists() {
        println!("[🟩] Binary Status: Found compiled xrizer libraries in target directory.");
    } else {
        println!("[❌] Binary Status: Directory exists, but NO COMPILED .so FILES FOUND!");
        println!("    └── Did a clean build clear the directory? Try running 'cargo build --release' inside your xrizer folder.");
    }

    // 2. Cross-reference with Steam parameters by checking VRChat command line arguments
    for (pid, proc) in sys.processes() {
        if proc.name().to_string().to_lowercase().contains("vrchat") {
            println!("[🎮] Analyzing Steam's handoff to VRChat (PID: {})...", pid);
            verify_launch_arguments(proc.cmd());
        }
    }
}


//fn verify_launch_arguments<T: AsRef<str>>(cmd_args: &[T]) {
fn verify_launch_arguments(cmd_args: &[String]) {
// This was originally written to take in an OsString and process it into a string, but sysinfo::proc.cmd() outputs clean strings.
    let mut args_string = String::new();

    // Check if Steam actually appended the override path to the execution string
    if args_string.contains("VR_OVERRIDE") || args_string.contains("xrizer") {
        println!("    ├── [🟩] Steam Launch Check: Steam tried to pass xrizer to the execution call.");
        println!("    └── [⚠️] Diagnosis: Steam passed it, but the srt-bwrap sandbox stripped it out.");
    } else {
        println!("    └── [❌] Steam Launch Check: VR_OVERRIDE is entirely missing from the execution arguments!");
        println!("        └── Action: Re-check your VRChat Steam Launch Options string for syntax typos.");
    }
}
