use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        let program = args.first().map(|s| s.as_str()).unwrap_or("turbo-log-insight");
        eprintln!("使い方: {} <検索キーワード> <ファイル名>", program);
        process::exit(1);
    }

    let keyword = &args[1];
    let path = &args[2];
    if let Err(e) = run(keyword, path) {
        eprintln!("エラー: {}", e);
        process::exit(1);
    }
}

fn run(keyword: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result?;
        if line.contains(keyword) {
            println!("{}", line);
        }
    }

    Ok(())
}
