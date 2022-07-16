use std::io::Error;
use std::io::ErrorKind;
use unshare::Namespace;
use std::io::{stderr, Write, Read};



pub struct Runtime {
    pub cmd: String,
    pub args: Vec<String>,
    pub rootfs: String
}

impl Runtime {
    pub fn run(self) -> Result<unshare::ExitStatus, Error> {

        // Construct and isolate the container
        let mut command = unshare::Command::new(self.cmd);
        command.args(&self.args);
        command.gid(1001);
        command.uid(1001);
        command.chroot_dir(self.rootfs);
        apply_namespaces(&mut command);
        command.close_fds(..);

        // Spawn the container process
        let mut container_process = match command.spawn() {
            Ok(container_process) => { container_process }
            Err(e) => {
                return Err(Error::new(ErrorKind::Other, "Could not spawn container process"));
            }
        };

        // Stream the stdout of the container child process to the stderr of this process
        let mut container_process_stdout = Vec::new();
        container_process.stdout.take().unwrap().read_to_end(&mut container_process_stdout).unwrap();
        writeln!(&mut stderr(), "{:?}", String::from_utf8_lossy(&container_process_stdout[..])).unwrap();

        // Return the status code once the container process is complete
        let result = container_process.wait()?;
        return Ok(result);
       
    }
}


fn apply_namespaces(command: &mut unshare::Command) {
    let mut namespaces = Vec::<Namespace>::new();
    namespaces.push(Namespace::Net);
    namespaces.push(Namespace::Net);
    namespaces.push(Namespace::Mount);
    namespaces.push(Namespace::Uts);
    namespaces.push(Namespace::Ipc);
    namespaces.push(Namespace::User);
    command.unshare(&namespaces);
}