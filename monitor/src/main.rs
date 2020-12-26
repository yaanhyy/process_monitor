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

fn scan_process_info(process_name: &str, mut process_infos: Vec<ProcessInfo>) ->  Vec<ProcessInfo> {
    let output= Command::new("ps")
        .arg("ax")
        .output()
        .expect("failed to execute process");
    let st = String::from_utf8_lossy(&output.stdout);
    let lines = st.split("\n");

    let mut ori_pids: Vec<String> = Vec::new();
    let mut pid_index_map = HashMap::new();
    let size = process_infos.len();

    let mut proces_scan: Vec<ProcessInfo> = Vec::new();

    for  i in 0..size {
        let  info = process_infos.get(i).unwrap();
        let pid = info.pid.clone();
        ori_pids.push(pid.clone());
        pid_index_map.insert(pid, i);
        proces_scan.push(ProcessInfo{pid: info.pid.clone(), arg: info.arg.clone(), status:"killed".to_string()})
    }
    // for mut info in &process_infos {
    //     ori_pids.push(info.pid.clone());
    //     (*info).status = "killed".to_string();
    // }
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
                     proces_scan.push(ProcessInfo { pid, arg, status});
                 } else {
                     let info = proces_scan.get_mut(*pid_index_map.get_mut(&pid).unwrap()).unwrap();
                     info.status = "running".to_string();
                 }

             }
        }
    }

    let size = process_infos.len();
    for  i in 0..size {
        println!("proc index:{}", i);
        let  info = process_infos.get_mut(i);
        if info.is_none() {
            continue;
        }
        let info = info.unwrap();
        if info.status.contains("killed") {
            println!("killed proc:{:?}", info);

            let res = Command::new("forever")
                .arg(&info.arg).spawn()
                .expect("failed to execute process");
            process_infos.remove(i);
            println!("restart:{:?}", res);

        }

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
    let mut process_infos = Vec::new();
    while(true) {
        process_infos = scan_process_info(&input, process_infos);

        thread::sleep(std::time::Duration::from_millis(1000));
        for info in &process_infos {
            println!("proc:{:?}", info);
        }
    }
    println!("Hello, world!");
}
