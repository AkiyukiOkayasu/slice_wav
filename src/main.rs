use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    input: std::path::PathBuf,

    /// Name of the person to greet
    #[arg(short, long)]
    name: Option<String>,
}

fn main() {
    let args = Args::parse();

    let mut export_path = PathBuf::from(args.input.clone());
    export_path.pop();
    export_path.push("export");
    let metadata = args.input.clone().metadata().unwrap();
    assert_eq!(false, metadata.permissions().readonly());

    if !export_path.exists() {
        match fs::create_dir(export_path.as_path()) {
            Err(e) => eprintln!("Failed on create \"export\" dir : {}", e),
            Ok(_) => {}
        }
    }

    println!(
        "Export folder: {}",
        export_path.canonicalize().unwrap().display()
    );

    let output = Command::new("pwd").output().expect("failed");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("");
    let output = Command::new("ls").args(&["-a"]).output().expect("failed");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
