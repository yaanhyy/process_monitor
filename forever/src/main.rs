use std::{io, thread, time::Duration, env, convert::AsRef};
use fork::{fork, daemon, Fork};
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut input = Default::default();
    if args.len() == 2 {
        input = (&args[1]).clone();
    } else {
        println!("only need one arg used as pid name");
        return;
    }
    println!("input arg:{}", input);
    if let Ok(Fork::Child) = daemon(true, true) {
        while(true) {
            Command::new("sleep")
                .arg("3")
                .output()
                .expect("failed to execute process");
        }
    }
    println!("exit!");
}

#[test]
fn comand_test() {
    let mut pid_name = String::new();
    println!("init");


    while(true) {
        io::stdin()
            .read_line(&mut pid_name)
            .expect("Failed to read line");

        if pid_name.eq("exit") {
            break;
        }
        println!("You input process name is: {}", pid_name);

        Command::new("forever")
            .arg(&pid_name).spawn()
            .expect("failed to execute process");

    }
}
