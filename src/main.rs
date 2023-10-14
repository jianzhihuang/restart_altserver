use std::process::{Command, exit};
use std::{env, thread};
use std::path::Path;
use std::time::Duration;

fn main() {
    println!("Finding AltServer.exe process...");
      println!("Finding and killing existing AltServer.exe process...");

    let _output = Command::new("taskkill")
        .args(&["/F", "/IM", "AltServer.exe"])
        .output()
        .expect("Failed to execute command");

    // Wait for kill to take effect
    for _ in 0..2{
        let output = Command::new("tasklist")
            .output()
            .expect("Failed to execute command");
        let output = String::from_utf8_lossy(&output.stdout).into_owned();

        if !output.contains("AltServer.exe") {
            break;
        }
        println!("Waiting for AltServer.exe to terminate...");
        thread::sleep(Duration::from_secs(5))
    }

     let output = Command::new("cmd")
        .args(&["/C", "chcp 65001>nul && tasklist"])
        .output()
        .expect("Failed to execute command");

    let output =
        match String::from_utf8(output.stdout) {
        Ok(v) => v,
        Err(e) => {
            println!("Failed to parse output: {}", e);
            return;
        }
    };
    let pid = output.lines()
        .filter_map(|line| line.split_whitespace().nth(1))
        .find(|word| word.ends_with("AltServer.exe"));

    match pid {
        Some(pid) => {
            // println!("Found AltServer.exe process, PID is {}.", pid);
            println!("Killing process...");

            Command::new("taskkill")
                .arg("/F")
                .arg(format!("/PID {}", pid))
                .output()
                .expect("Failed to execute command");

            println!("Process killed.");
        }

        None => {
            println!("AltServer.exe process not found.");
        }
    }

    println!("Starting AltServer.exe...");

    let path = Path::new(r"C:\Program Files (x86)\AltServer\AltServer.exe");
    if path.exists() {
        Command::new(path)
            .spawn()
            .expect("Failed to start AltServer.exe");
        println!("AltServer.exe started.");
    } else {
        println!("Cannot found AltServer.exe in the specified path");
        exit(1);
    }

    println!("Done.");
}