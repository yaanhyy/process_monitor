use std::{io, thread, time::Duration, env, convert::AsRef};
use std::process::Command;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ProcessInfo {
    pub pid: String,
    pub arg: String,
    pub status: String,  //runing, killed, restart
}

fn scan_process_info(process_name: &str, mut process_infos: HashMap<String,ProcessInfo>) ->  HashMap<String,ProcessInfo> {
    let output= Command::new("ps")
        .arg("ax")
        .output()
        .expect("failed to execute process");
    let st = String::from_utf8_lossy(&output.stdout);
    let lines = st.split("\n");

    let mut ori_pids: Vec<String> = Vec::new();


    let mut proces_scan: HashMap<String,ProcessInfo> = HashMap::new();

    //proces_scan = process_infos.iter_mut().map(|info| {info.1.status="killed".to_string(); return info}).collect();
    let mut proc_keys = process_infos.keys();
    for key in proc_keys {
        let info = process_infos.get(key).unwrap();
        //info.status = "killed".to_string();
        proces_scan.insert(key.clone(), ProcessInfo{pid: info.pid.clone(), arg: info.arg.clone(), status:"killed".to_string()});
    }

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
                     proces_scan.insert(pid.clone(),ProcessInfo { pid, arg, status});
                 } else {
                     let info = proces_scan.get_mut(&pid).unwrap();
                     info.status = "running".to_string();
                 }

             }
        }
    }


    let mut proc_keys = proces_scan.keys();
    let mut killed_keys = Vec::new();
    for  key in proc_keys {

        let  info = proces_scan.get(key);
        println!("proc index:{}, proc info:{:?}", key, info);
        if info.is_none() {
            continue;
        }
        let info = info.unwrap();
        if info.status.contains("killed") {
            println!("killed proc:{:?}", info);
            killed_keys.push(key.clone());
            let res = Command::new("forever")
                .arg(&info.arg).spawn()
                .expect("failed to execute process");

            println!("restart:{:?}", res);

        }

    }

    for key in killed_keys {
        proces_scan.remove(&key);
    }
    return proces_scan;
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
    let mut process_infos = HashMap::new();
    while(true) {
        process_infos = scan_process_info(&input, process_infos);

        thread::sleep(std::time::Duration::from_millis(1000));
        for info in &process_infos {
            println!("proc:{:?}", info);
        }
    }
    println!("Hello, world!");
}
