use std::{io, thread, time::Duration, env, convert::AsRef};
use std::process::Command;

fn get_process_info(process_name: &str) {
    Command::new("ps")
        .arg(&pid_name)
        .output()
        .expect("failed to execute process");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut input = Default::default();
    if args.len() == 2 {
        input = (&args[1]).clone();
    } else {
        println!("only need one arg used as pid name");
        return;
    }
    println!("input process name:{}", input);



    println!("Hello, world!");
}
