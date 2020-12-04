#![deny(warnings)]

use std::process::Command;
use std::time::SystemTime;

const N_DAYS: u8 = 4;

fn bin_name(day: u8) -> String {
    format!("{:02}", day)
}

fn build_all(days: impl Iterator<Item = u8>) {
    println!("Building:");
    let args = days
        .flat_map(|day| vec!["-p".to_string(), bin_name(day)])
        .collect::<Vec<_>>();
    Command::new("cargo")
        .arg("build")
        .arg("--release")
        .args(args.iter())
        .status()
        .expect("Building failed");
}

fn run(day: u8) {
    println!("=== Day {}:        ===", bin_name(day));

    let start_time = SystemTime::now();

    Command::new("cargo")
        .arg("run")
        .arg("--release")
        .arg("--quiet")
        .arg("-p")
        .arg(&bin_name(day))
        .status()
        .unwrap_or_else(|_| panic!("Running {} failed", bin_name(day)));

    let elapsed = start_time.elapsed().unwrap();
    println!(
        "=== Done ({:01}s{:3}ms) ===\n",
        elapsed.as_secs(),
        elapsed.subsec_millis()
    );
}

fn main() {
    let n_days = N_DAYS;
    build_all(1..=n_days);
    let start_time = SystemTime::now();
    for day in 1..=n_days {
        run(day);
    }
    let elapsed = start_time.elapsed().unwrap();
    println!("Total time: {:?}\n", elapsed);
}
