use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::str;
use std::time::{SystemTime, UNIX_EPOCH};

fn executable_path() -> PathBuf {
    let mut path = env::current_exe().unwrap();
    path.pop();
    path.pop();
    path.join("orthanc")
}

fn run_command(args: Vec<&str>) {
    Command::new(executable_path())
        .args(&args)
        .output()
        .unwrap();
}

#[test]
fn test_slow() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    run_command(vec!["--server", "http://localhost:8901", "patient", "list"]);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Test run time: {}", end.as_millis() - start.as_millis());
}
