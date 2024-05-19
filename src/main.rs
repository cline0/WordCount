use std::{
    fs::File,
    io::{self, BufRead},
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// 是否单词计数
    #[clap(short, long, default_value = "true")]
    word: bool,
    /// 是否字符计数
    #[clap(short, long, default_value = "false")]
    char: bool,
    /// 进行计数的文件
    #[clap(required = true)]
    file: String,
}
fn main() -> io::Result<()> {
    let args = Args::parse();
    let file = File::open(&args.file)?;
    let reader = io::BufReader::new(&file);

    let mut char_count = 0;
    let mut word_count = 0;
    let mut last_line_ended_with_newline = true;

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            // 空行
            if args.char {
                char_count += 1;
            }
            last_line_ended_with_newline = true;
            continue;
        }

        word_count += line.split(&[' ', ',', '\t']).count();
        if args.char {
            // 有一个多余的换行符
            char_count += line.len() + 1;
        }
        last_line_ended_with_newline = false;
    }

    if args.char && !last_line_ended_with_newline {
        // 如果最后一行没有以换行符结束，那么我们需要加上一个换行符
        char_count += 1;
    }

    println!("单词数：{}", word_count);
    if args.char {
        println!("字符数：{}", char_count);
    }
    Ok(())
}
