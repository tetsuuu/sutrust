use std::fs;
use std::fs::*;
use std::fs::File;
// use std::io::Write;
// use std::io::Read;
use std::io::{Read, Write, BufRead, BufReader, BufWriter};
use std::env;

fn main() {
    // ファイルの読み込み
    let path = "sample.txt";
    println!("read all lines.");
    if let Ok(data) = std::fs::read_to_string( path ) {
        println!("data is {}", data);
    }
    // 書き方2 use std::fs;
    println!("read all lines");
    if let Ok(data) = fs::read_to_string( path ) {
        println!("data is {}", data);
    } else {
        println!("cannot open {}", path);
    }
    // match文を使ったエラー時の挙動を追加
    println!("read all lines.");
    match fs::read_to_string( path ) {
        Ok(data) => { println!("data is {}", data); },
        _ => { println!("cannot open {}", path )}
    }

    // 書き方3 use std::fs::*;
    println!("read all lines");
    if let Ok(data) = read_to_string( path ) {
        println!("data is {}", data);
    }

    // バッファに文字列を読み込む
    println!("read all lines by buffer.");
    let mut file = File::open( path )
        .expect("file not found.");
    let mut data = String::new();
    file.read_to_string( &mut data )
        .expect("read error.");
    println!("data is {}", data);

    // ファイルアクセス時のResultをチェックする
    println!("read all lines by buffer.");
    if let Ok(mut file) = File::open( path ) {
        let mut data = String::new();
        if let Ok(_) = file.read_to_string( &mut data ) {
            println!("data is {}", data);
        }
    }

    // バイト単位でファイルを読み込む
    println!("read 16 byte by buffer.");
    let mut file = File::open( path )
        .expect("file not found.");
    let mut buf : [u8; 1] = [0; 1];
    for i in 0..16 {
        file.read(&mut buf);
        println!("buf is {}: {}", i, buf[0] as char);
    }

    // 一行ずつ読む
    let file = File::open( path )
        .expect("file not found.");
    for line in BufReader::new(file).lines() {
        if let Ok(1) = line {
            println!("line us {}", 1);
        }
    }

    // ファイルに文字列を書き出す
    let path = "sample.txt";
    let mut file = File::create( path )
        .expect("file not found.");
    writeln!( file, "hello rust world.")
        .expect("cannot write.");

    // バイト単位でファイルに書き込む
    let mut file = File::create( path )
        .expect("file not found");
    file.write( b"hello rust world.\n" )
        .expect("cannot write.");

    // バイト配列で書き出す
    let s = "hello rust world.\n";
    file.write(s.as_bytes())
        .expect("cannot write.");
    
    // 1バイト単位での書き込み
    let s = "hello rust world.\n";
    for it in s.as_bytes() {
        file.write(&[*it])
            .expect("cannot write.");
    }

    // 1バイト単位での書き込み2
    let s = "hello rust world.\n";
    for it in s.as_bytes() {
        let ch = *it;
        let ary = [ch];
        file.write(&ary)
            .expect("cannot write.");
    }

    // コマンドに引数を一つ渡してからファイルを読み込む
    let args = env::args().collect::<Vec<String>>();
    if args.len() <= 1 {
        // 標準入力をそのまま書き出す
    } else {
        // ファイル指定の場合はファイルをオープンする
        let file = File::open( &args[1] )
            .expect("file not found.");
        let reader = BufReader::new(file);
        let mut writer = BufWriter::new(std::io::stdout());
        for it in reader.bytes() {
            if let Ok(n) = it {
                writer.write( &[n] );
            }
        }
    }

    // 標準入力を使ったコード
    let args = env::args().collect::<Vec<String>>();
    if args.len() <= 1 {
        let reader = BufReader::new(std::io::stdin());
        let mut writer = BufWriter::new(std::io::stdout());
        for it in reader.bytes() {
            if let Ok(n) = it {
                writer.write( &[n] );
            }
        }
    } else {
        let file = File::open( &args[1] )
            .expect("file not found.");
        let reader = BufReader::new(file);
        let mut writer = BufWriter::new(std::io::stdout());
        for it in reader.bytes() {
            if let Ok(n) = it {
                writer.write( &[n] );
            }
        }
    }

    // do_printを利用して共通化
    let args = env::args().collect::<Vec<String>>();
    if args.len() <= 1 {
        let reader = BufReader::new(std::io::stdin());
        do_print( reader );
    } else {
        let file = File::open( &args[1] )
            .expect("file not found.");
        let reader = BufReader::new(file);
        do_print( reader );
    }
}

fn do_print<R>( reader: BufReader<R>) where R: std::io::Read {
    let mut writer = BufWriter::new(std::io::stdout());
    for i in reader.bytes() {
        if let Ok(n) = i {
            writer.write(&[n]);
        }
    }
}
// main関数でファイルを読む
// fn main() -> std::io::Result<()> {
//     let path = "sample.txt";
//     let file = File::open( path )?;
//     for line in BufReader::new(file).lines() {
//         println!("line is {}", line?);
//     }
// }
