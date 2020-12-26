use std::process::Command;
use std::{io, thread, time::Duration, env, convert::AsRef};

fn main() {
    let mut pid_name = String::new();
    println!("init");


    while(true) {
        io::stdin()
            .read_line(&mut pid_name)
            .expect("Failed to read line");

        if pid_name.eq("exit\n") {
            break;
        }
        println!("You input process name is: {}", &pid_name.to_string());

        Command::new("forever")
            .arg(&pid_name).spawn()
            .expect("failed to execute process");
        pid_name.clear();

    }
}
