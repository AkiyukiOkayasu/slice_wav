use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    input: std::path::PathBuf,

    #[arg(short, long, default_value = "0")]
    start: u32,

    #[arg(short, long, default_value = "100")]
    length: u32,

    #[arg(long, default_value = "10")]
    interval: u32,

    /// Name of the person to greet
    #[arg(short, long)]
    name: Option<String>,
}

fn main() {
    let args = Args::parse();
    let input = args.input;
    is_exit_file(&input); //inputが存在するか確認
    is_wav_file(&input); //inputがwavファイルか確認
    let input = input.canonicalize().unwrap(); //絶対パスに変換

    // exportフォルダを作成
    let mut export_path = create_export_folder(&input);
    println!(
        "Export folder: {}",
        export_path.canonicalize().unwrap().display()
    );

    export_path.push("out.wav");
    println!("Export file: {}", export_path.display());

    let output = Command::new("sox")
        .args(&[
            input.as_os_str().to_str().unwrap(),
            export_path.as_os_str().to_str().unwrap(),
            "trim",
            "10s",
            "20s",
        ])
        .output()
        .expect("failed");
    println!("{:?}", output);
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn create_export_folder(path: &PathBuf) -> PathBuf {
    let mut export_path = PathBuf::from(path.clone());
    export_path.pop();
    export_path.push("export");
    let metadata = path.clone().metadata().unwrap();
    assert_eq!(false, metadata.permissions().readonly());

    if !export_path.exists() {
        match fs::create_dir(export_path.as_path()) {
            Err(e) => eprintln!("Failed on create \"export\" dir : {}", e),
            Ok(_) => {}
        }
    }
    export_path
}

/// Pathで示されたファイルが存在するか
fn is_exit_file(path: &PathBuf) {
    if !path.exists() {
        eprintln!("{} is not found", path.display());
        std::process::exit(1);
    }
}

/// Pathがwavファイルかどうか
fn is_wav_file(path: &PathBuf) {
    match path.extension() {
        Some(x) => {
            if x.is_empty() {
                eprintln!("Input file extension is must to be \"wav\"");
                std::process::exit(1);
            }

            match x.to_str().unwrap() {
                "wav" => {}
                "WAV" => {}
                _ => {
                    eprintln!("Input file extension is must to be \"wav\"");
                    std::process::exit(1);
                }
            }
        }
        None => {}
    }
}
