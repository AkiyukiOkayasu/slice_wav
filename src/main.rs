use clap::Parser;
use hound;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input WAV file to slice.
    input: std::path::PathBuf,

    /// Start position in samples. Default is 0.
    #[arg(short, long, default_value = "0")]
    start: u32,

    /// Length of the exported WAV file in samples.
    #[arg(short, long)]
    length: u32,

    /// Interval to slice in samples.
    #[arg(long)]
    interval: Option<u32>,
}

fn main() {
    let args = Args::parse();
    let input = args.input;
    is_exit_file(&input); //inputが存在するか確認
    is_wav_file(&input); //inputがwavファイルか確認
    let input = input.canonicalize().unwrap(); //絶対パスに変換

    let reader = hound::WavReader::open(input.clone()).unwrap();
    let num_samples = reader.duration();
    println!("Number of samples per channel: {}", num_samples);

    // exportフォルダを作成
    let export_path = create_export_folder(&input);
    println!(
        "Export folder: {}",
        export_path.canonicalize().unwrap().display()
    );

    let interval = match args.interval {
        Some(x) => x,
        None => args.length,
    };
    dbg!(interval);

    let mut count = 0;
    loop {
        let mut ex = export_path.clone();
        let start_sample = args.start + interval * count;
        let end_sample = start_sample + args.length;

        if end_sample > num_samples {
            break;
        }

        ex.push(format!(
            "out{:03}_start{}_len{}.wav",
            count, start_sample, args.length
        ));
        let output = Command::new("sox")
            .args(&[
                input.as_os_str().to_str().unwrap(),
                ex.as_os_str().to_str().unwrap(),
                "trim",
                &format!("{}s", start_sample),
                &format!("{}s", args.length),
            ])
            .output()
            .expect("failed");
        println!("{:?}", output);
        println!("{}", String::from_utf8_lossy(&output.stdout));

        count += 1;
    }
}

/// 出力先フォルダを作成
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
