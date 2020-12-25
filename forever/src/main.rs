use std::{io, thread, time::Duration, env, convert::AsRef};

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




    while(true) {
        let mut pid_name = String::new();
        io::stdin()
            .read_line(&mut pid_name)
            .expect("Failed to read line");

        println!("You input process name is: {}", pid_name);
        thread::spawn(move || {
            println!("process run:{}", &pid_name);
           while(true) {


           }
        });
    }

    println!("exit!");
}
