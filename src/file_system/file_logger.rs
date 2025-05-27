
/*

file_logger: Build a simple logging system that writes logs to a file.
Tasks:

Create or open a file

Write timestamped log entries

Append logs without overwriting

Close the file properly

*/


use std::{fs::{File, OpenOptions}, io::{Write}};
use std::time::{Duration};
use std::thread;
use chrono::Local;
const LOG_FILE_NAME: &str = "logs.log";

fn open_or_create_log_file() -> File {
  OpenOptions::new()
  .create(true)
  .append(true)
  .open(format!("{}", LOG_FILE_NAME))
  .expect("Failed to open or create log file")
}

fn sleep(ms: u64) {  
  thread::sleep(Duration::from_millis(ms));
}

fn get_current_time() -> String {
 Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

fn logln<T: std::fmt::Display>(msg: T) -> String {
  return format!("LOG: {}\n", msg);
}

pub fn start() {
    let mut file = open_or_create_log_file(); 

    //write logs
    println!("Writing logs");
    loop {      
      sleep(200);
      let time_now = get_current_time();
      let _ = file.write(logln(time_now).as_bytes());
      println!("Log Written");
    }
}





