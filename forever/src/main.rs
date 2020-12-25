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




    while(true) {
        let mut pid_name = String::new();
        io::stdin()
            .read_line(&mut pid_name)
            .expect("Failed to read line");

        println!("You input process name is: {}", pid_name);

        /// spawn process
        match fork() {
            Ok(Fork::Parent(child)) => {
                println!("Continuing execution in parent process, new child has pid: {}", child);
            }
            Ok(Fork::Child) => {
               // if let Ok(Fork::Child) = daemon(false, false) {
                    println!("process run:{}", &pid_name);
                    while(true) {
                        let ten_millis = Duration::from_millis(1000);


                        thread::sleep(ten_millis);
                    }
               // }
            },
            Err(_) => println!("Fork failed"),
        }


        println!("parent loop!");

        // spawn thread
        // thread::spawn(move || {
        //     println!("process run:{}", &pid_name);
        //    while(true) {
        //
        //
        //    }
        // });
    }

    println!("exit!");
}
