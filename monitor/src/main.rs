use std::{io, thread, time::Duration, env, convert::AsRef};
use std::process::Command;


struct ProcessInfo {
    pid: u128,
    arg: String
}

fn get_process_info(process_name: &str) -> Vec<ProcessInfo>{
    let output= Command::new("ps")
        .arg("ax")
        .output()
        .expect("failed to execute process");
    let st = String::from_utf8_lossy(&output.stdout);
    let lines = st.split("\n");

    let process_infos = Vec::new();
    for line in lines {
        if line.contains(process_name) {
            let line_clone = line.to_string().clone();
            let process_info_fields: Vec<_> = line_clone.split(' ').filter(|&x| !x.is_empty()).collect();
            let proc_name = process_info_fields.get(4).unwrap().to_string();
            println!("proc name:{:?}", process_info_fields);
             if (&proc_name).eq(process_name) {
                 println!("pid:{:?}", process_info_fields.get(0));
                 println!("arg:{:?}", process_info_fields.get(5));
             }
        }
    }
    return process_infos;
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

    get_process_info(&input);

    println!("Hello, world!");
}
