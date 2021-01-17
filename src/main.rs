use dotenv::dotenv;
use std::env::args;
use std::path::Path;

mod server;
mod sys;

const SERVER_PID_DIR: &'static str = "./server.pid";

fn main() {
    dotenv().ok();
    let mut stop: bool = false;
    let mut start: bool = false;
    for argument in args() {
        if argument == "stop" {
            stop = true;
        }

        if argument == "start" {
            start = true;
        }
    }

    let is_file = Path::new(SERVER_PID_DIR).is_file();
    if stop {
        if is_file {
            sys::kill(SERVER_PID_DIR);
        } else {
            println!("pid file not found");
        }
    }

    if start {
        if is_file {
            println!("server exists");
        } else {
            sys::fork(SERVER_PID_DIR)
        }
    }
}
