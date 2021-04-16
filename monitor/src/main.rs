use std::{io, thread, time::Duration, env, convert::AsRef};
use std::process::Command;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use sysinfo::{ProcessExt, System, SystemExt};
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ProcessInfo {
    pub pid: String,
    pub arg: String,
    pub status: String,  //runing, killed, restart
}

fn scan_process_info(process_name: &str, mut process_infos: & mut HashMap<String,ProcessInfo>)  {
    let output= Command::new("ps")
        .arg("ax")
        .output()
        .expect("failed to execute process");
    let st = String::from_utf8_lossy(&output.stdout);
    let lines = st.split("\n");

    //init process infos before scan,set status to killed
    let mut ori_pids: Vec<String> = Vec::new();
    process_infos.iter_mut().for_each(|(_,info)| {info.status="killed".to_string();ori_pids.push(info.pid.clone())});

    for line in lines {
        if line.contains(process_name) {
            let line_clone = line.to_string().clone();
            let process_info_fields: Vec<_> = line_clone.split(' ').filter(|&x| !x.is_empty()).collect();
            let proc_name = process_info_fields.get(4).unwrap().to_string();
            //println!("proc name:{:?}", process_info_fields);
             if (&proc_name).eq(process_name) {
                 let pid= process_info_fields.get(0).unwrap().to_string();
                 println!("pid:{:?}", pid);
                 let arg= process_info_fields.get(5);
                 if arg.is_none() {
                     continue;
                 }
                 let arg =    arg.unwrap().to_string();
                 println!("arg:{:?}", arg);
                 if !ori_pids.contains(&pid) {
                     let status = "running".to_string();
                     process_infos.insert(pid.clone(),ProcessInfo { pid, arg, status});
                 } else {
                     let info = process_infos.get_mut(&pid).unwrap();
                     info.status = "running".to_string();
                 }

             }
        }
    }


    let mut proc_keys = process_infos.keys();
    let mut killed_keys = Vec::new();
    for  key in proc_keys {

        let  info = process_infos.get(key);
        println!("proc index:{}, proc info:{:?}", key, info);
        if info.is_none() {
            continue;
        }
        let info = info.unwrap();
        if info.status.contains("killed") {
            println!("killed proc:{:?}", info);
            killed_keys.push(key.clone());
            let res = Command::new(process_name)
                .arg(&info.arg).spawn()
                .expect("failed to execute process");

            println!("restart:{:?}", res);
        }
    }

    for key in killed_keys {
        process_infos.remove(&key);
    }

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
    let s = System::new_all();
    for (pid, process) in s.get_processes() {
        println!("{} {} {:?}", pid, process.name(), process.cmd());
    }
    println!("input process name:{}", input);
    let mut process_infos = HashMap::new();
    while(true) {
        scan_process_info(&input, & mut process_infos);

        thread::sleep(std::time::Duration::from_millis(8000));
        for info in &process_infos {
            println!("proc:{:?}", info);
        }
    }
    println!("Hello, world!");
}
