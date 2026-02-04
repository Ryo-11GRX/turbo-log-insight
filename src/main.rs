use anyhow::Result;
use clap::Parser;
// lib.rs で定義したモジュールを使うための宣言
use turbo_log_insight::config::Config;
use turbo_log_insight::run;

fn main() -> Result<()> {
    // 1. 引数をパース（ここで所有権が発生）
    let config = Config::parse();

    // 2. ロジックを実行
    // ここで config の所有権が run 関数に「ムーブ（移動）」します。
    // 以降、main関数内で config は使えなくなります（借用エラー防止）。
    if let Err(e) = run(config) {
        // エラー時は標準エラー出力(stderr)に表示
        eprintln!("Application Error: {:#}", e);
        std::process::exit(1);
    }

    Ok(())
}