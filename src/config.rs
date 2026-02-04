use clap::Parser;

/// プログラムの設定情報（CLI引数から生成）
/// 
/// メンター解説:
/// ここで `String` を使っているのは「所有権」を持つためです。
/// ファイルパスや検索クエリは、プログラムが動いている間ずっとメモリ上に
/// 存在してほしいデータなので、借用(&str)ではなく所有(String)します。
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// 検索したいキーワード
    #[arg(short, long)]
    pub query: String,

    /// 解析対象のログファイルパス
    #[arg(short, long)]
    pub file_path: String,
}
