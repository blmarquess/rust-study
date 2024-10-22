use std::process::Command;

// fn run_command(command: &str) -> String {
//     let args: Vec<&str> = command.split(" ").collect();
//     let output = Command::new(args[0])
//         .args(&args[1..])
//         .output()
//         .expect("Failed to execute command");
//     let stdout = String::from_utf8_lossy(&output.stdout);
//     stdout.to_string()
// }

// fn run_lsblk(device: &str) -> serde_json::Value {
//     let command = "lsblk -J -o NAME,SIZE,TYPE,MOUNTPOINT";
//     let output = run_command(command);
//     println!("{:?}", output);
//     let devices: serde_json::Value = serde_json::from_str(&output).unwrap();
//     let devices = devices["blockdevices"].as_array().unwrap();
//     for parent in devices {
//         if parent["name"] == device {
//             return parent.clone();
//         }
//         if let Some(children) = parent["children"].as_array() {
//             for child in children {
//                 if child["name"] == device {
//                     return child.clone();
//                 }
//             }
//         }
//     }
//     panic!("Device not found");
// }

fn run_git_status() {
    let git_st = Command::new("git status")
        .output()
        .expect("erro on command git!");
    print!("{:?}", git_st);
}

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    // let device = args.last().unwrap();
    // let output = serde_json::to_string(&run_lsblk(&device)).unwrap();
    // println!("{}", output);
    args.reverse();
    args.pop();
    println!("{:?}", args);
    run_git_status()
}
