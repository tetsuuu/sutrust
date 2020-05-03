fn main() {
    // 文字列を扱う
    let ch = 'A';
    println!("ch is {}", ch);
    let ch = 'あ';
    println!("ch is {}", ch);
    let ch = '🐱';
    println!("ch is {}", ch);
    let ch = '🐶';
    println!("ch is {}", ch);

    // 通常の文字列の表示
    let s = "hello rust world.";
    println!("s is {}", s);

    // 部分的に文字列を取り出す
    let s = "hello rust world.";
    let hello = &s[0..5];
    let world = &s[11..];
    println!("hello is {}", hello);
    println!("world is {}", world);

    // 文字列の長さを取得する
    let s = "hello rust world.";
    let len = s.len();
    println!("s.len is {}", len);

    // 文字列の連結
    let mut s = String::new();
    // 文字列の追加
    s.push_str("hello ");
    s.push_str("rust ");
    s.push_str("world.");
    println!("s is {}", s);

    // format!マクロを使った連結
    let hello = "HELLO";
    let rust = "RUST";
    let world = "WORLD";
    let s = format!("{} {} {}", hello, rust, world);
    println!("s is {}", s);

    // &String型の文字列をつくる
    let s = "hello rust world.".to_string();
    println!("s is {}", s);
    let s = String::from("hello rust world.");
    println!("s is {}", s);

    // 日本語の扱い
    let s = "こんにちは rust コードの世界";
    println!("s is {}", s);

    // 実行時のエラー例
    let s = "こんにちは rust コードの世界";
    let hello = &s[0..5];
    let world = &s[11..];
    println!("こんにちは is {}", hello);
    println!("コードの世界 is {}", world);

    // 正しい例 ひらがなは3バイトに割り当てられる
    let s = "こんにちは rust コードの世界";
    let hello = &s[0..15];
    let world = &s[21..];
    println!("こんにちは is {}", hello);
    println!("コードの世界 is {}", world);

    // 文字列の長さを取得する
    let s = "こんにちは rust コードの世界";
    let len = s.len();
    println!("s.len is {}", len);
    // s.len is 39

    // 日本語の連結
    let mut s = String::new();
    s.push_str("こんにちは ");
    s.push_str("RUST ");
    s.push_str("コードの世界");
    println!("s is {}", s);

    // 日本語をformat!マクロでつなげた場合もOK
    let hello = "こんにちは ";
    let rust = "RUST ";
    let world = "コードの世界";
    let s = format!("{} {} {}", hello, rust, world);
    println!("s is {}", s);

    // &str型から&String型への変換
    let s = "こんにちは rust コードの世界".to_string();
    println!(" s is {}", s);
    let s = String::from("こんにちは rust コードの世界");
    println!("s is {}", s);

    // 文字列からchar型のベクター型を作成する
    let s = "This is ねこ🐱neko 文字列";
    // ベクターに直す
    let mut v : Vec<char> = Vec::new();
    for c in s.chars() {
        v.push(c);
    }
    // 8文字目から14文字目まで取得
    let v = &v[8..15];
    // 再度&Stringに直す
    let mut s = String::new();
    for c in v {
        s.push(*c)
    }
    println!("s is {}", s);

    // 文字列のスライス
    let s = " hello rust world.";
    // 先頭の文字
    let a = &s[0..1];
    println!("a is {}", a);
    // 先頭の5文字
    let a = &s[0..5];
    println!("a is {}", a);
    // 0を省略できる
    let a = &s[..5];
    println!("a is {}", a);

    // 文字列の途中を切り出す
    let s = "hello rust world.";
    // 途中の文字
    let a = &s[6..10];
    println!("a is {}", a);
    // 6文字目から4文字分取り出す
    let a = &s[6..(6+4)];
    println!("a is {}", a);

    // 最後までを切り出すスライス
    let s = "hello rust world.";
    // 11文字目から最後まで
    let a = &s[..11];
    println!("a is {}", a);
    // 明示的に最後を示す
    let n = s.len();
    let a = &s[11..n];
    println!("a is {}", a);
    // 範囲外だとpanicを起こす
    let a = &s[11..20];
    println!("a is {}", a);

    // 全体を切り出すスライス
    let s = "hello rust world.";
    // ..だけの場合は全体を返す
    let a = &s[..];
    println!("a is {}", a);
    // 明示的に全体を取得する
    let n = s.len();
    let a = &s[0..n];
    println!("a is {}", a);
    
}

// u8へのキャスト
fn test () {
    let ch = 'A';
    println!("ch is {}", ch);
    let u = ch as u8;
    println!("ch is {}", u);
    let ch = u as char;
    println!("ch is {}", ch);
}
