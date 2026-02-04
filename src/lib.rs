// モジュールを公開して使えるようにする
pub mod config;

use anyhow::{Context, Result};
use config::Config;
use std::fs;

/// アプリケーションの実行ロジック
///
/// # 引数
/// * `config`: 設定情報。
/// メンター解説: 
/// ここで `&Config` (参照) ではなく `Config` (実体) を受け取っている理由：
/// main関数はもうこのデータを使わないので、「所有権」ごとこの関数に移動(Move)させます。
/// これにより、run関数内で自由にデータを消費・加工できるようになります。
pub fn run(config: Config) -> Result<()> {
    // ファイル読み込み
    // メンター解説:
    // ?演算子は「エラーなら即座にreturn Err」をやってくれる魔法です。
    // context() は「どのファイルで失敗したか」という情報をエラーに追加します。
    // ※注意: read_to_stringはファイル全体をメモリに読むため、巨大ファイルには向きません。
    // (次のステップでここを「1行ずつの読み込み」に改良します)
    let content = fs::read_to_string(&config.file_path)
        .with_context(|| format!("ファイルを読み込めませんでした: {}", config.file_path))?;

    println!("--------------------------------------------------");
    println!("検索ターゲット: {}", config.query);
    println!("対象ファイル: {}", config.file_path);
    println!("--------------------------------------------------");

    // 簡易的な検索ロジック（後で高速化します）
    for (i, line) in content.lines().enumerate() {
        if line.contains(&config.query) {
            println!("{}: {}", i + 1, line);
        }
    }

    Ok(())
}