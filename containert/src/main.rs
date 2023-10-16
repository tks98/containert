// Import necessary modules and libraries
use clap::{arg, Command};
use nix::sched::{unshare, CloneFlags};
use nix::sys::wait::wait;
use nix::unistd::{chdir, chroot, execvp, fork, ForkResult};
use std::ffi::CString;
use std::path::PathBuf;
use std::process::exit;
use cgroups_rs::{CgroupPid, hierarchies};
use cgroups_rs::cgroup_builder::CgroupBuilder;
use cgroups_rs::Controller;

// The main asynchronous function
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup and parse command-line arguments
    let matches = Command::new("containert")
        .arg(arg!(--binary_path <VALUE>).required(true))
        .arg(arg!(--rootfs <VALUE>).required(true))
        .get_matches();

    // Retrieve and store the path to the binary and root filesystem
    let binary_path = matches.get_one::<String>("binary_path").unwrap();
    let rootfs_path_arg = matches.get_one::<String>("rootfs").unwrap();

    let mut rootfs_path = PathBuf::new();
    rootfs_path.push(rootfs_path_arg);

    // Validate that the root filesystem path is actually a directory
    if !rootfs_path.is_dir() {
        eprintln!("Invalid rootfs directory");
        exit(1);
    }

    // Set up the flags for namespaces
    let flags = CloneFlags::CLONE_NEWNS | CloneFlags::CLONE_NEWPID | CloneFlags::CLONE_NEWUTS | CloneFlags::CLONE_NEWIPC | CloneFlags::CLONE_NEWNET;
    unshare(flags).unwrap();

    // Forking the process
    unsafe {
        match fork() {
            Ok(ForkResult::Parent { .. }) => {
                wait().unwrap();
                exit(0);
            },
            Ok(ForkResult::Child) => {
                // Child process code
                chroot(&rootfs_path).unwrap();
                chdir("/").unwrap();

                // Create and configure cgroups
                let hier = hierarchies::auto();
                let cg_result = CgroupBuilder::new("example")
                    .memory()
                    .kernel_memory_limit(4 * 1024 * 1024 * 1024)
                    .memory_hard_limit(4 * 1024 * 1024 * 1024)
                    .done()
                    .cpu()
                    .shares(100)
                    .done()
                    .build(hier);

                let cg = match cg_result {
                    Ok(cgroup) => cgroup,
                    Err(_) => {
                        eprintln!("Failed to create cgroup");
                        exit(1);
                    }
                };

                // Add the current process to the cgroup
                let cpus: &cgroups_rs::cpu::CpuController = cg.controller_of().unwrap();
                let memory: &cgroups_rs::memory::MemController = cg.controller_of().unwrap();
                let pid = CgroupPid::from(std::process::id() as u64);

                // Handle potential errors when adding tasks to cgroups
                if let Err(_) = cpus.add_task(&pid) {
                    eprintln!("Failed to add task to CPU cgroup");
                }
                if let Err(_) = memory.add_task(&pid) {
                    eprintln!("Failed to add task to Memory cgroup");
                }

                // Execute the command in the new environment
                let cmd = CString::new(binary_path.as_bytes()).unwrap();
                let args = [cmd.clone()];
                execvp(&cmd, &args).unwrap_or_else(|_| {
                    eprintln!("Failed to execute command");
                    exit(1);
                });
            },
            Err(_) => {
                eprintln!("Fork failed");
                exit(1);
            }
        }
    }
    Ok(())
}
