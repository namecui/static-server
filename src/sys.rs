use crate::server;
use nix::sys::signal;
use nix::unistd::{self, ForkResult, Pid};
use std::fs;
use std::io::prelude::*;

pub fn kill(path: &'static str) {
    let mut file = fs::File::open(path).expect("file open fail");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("reader fail");
    let id = contents.parse::<i32>().expect("pid type error");
    let get_pid = Pid::from_raw(id);
    signal::kill(get_pid, signal::SIGKILL).expect("kill fail");
    fs::remove_file(path).expect("remove fail");
    println!("exit success");
}

pub fn fork(path: &'static str) {
    match unsafe { unistd::fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            if let Ok(mut f) = fs::File::create(path) {
                let res = f.write_all(child.to_string().as_bytes());
                if res.is_err() {
                    println!("file write fail: {:?}", res)
                }
                println!("start success")
            } else {
                println!("file create  fail");
            }
        }
        Ok(ForkResult::Child) => {
            let _ = server::start();
        }
        Err(_) => println!("Fork failed"),
    }
}
