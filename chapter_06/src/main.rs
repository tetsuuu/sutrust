fn main() {
    // if分の例
    let a = 10;
    let b = 20;
    if a < b {
        println!("a < b is ok.")
    }
    // if分の後に括弧をつける
    let a = 10;
    let b = 20;
    // 括弧をつけているとwarningが出る
    if (a < b) {
        println!("a < b is ok.")
    }

    // if-els文の例
    let a = 10;
    let b = 20;
    // if-elseを使う
    if a < b {
        println!("a < b is ok.")
    } else {
        println!("a < b is ng.")
    }

    // 条件マッチしない場合のみは空ブロックを使う
    if a < b {
        // 空ブロック
    } else {
        pritnln!("a < b is ng.")
    }

    // 3パターンの条件を並べる場合
    let a = 10;
    let b = 20;
    // 複数のifを使う
    if a == b {
        println!("a == b is ok.")
    } else if a < b {
        println!("a < b is ok.")
    } else {
        println!("a > b is ok.")
    }

    // 論理積や論理和を使ったif文の例
    let a = 10;
    let b = 20;
    // 論理積
    if a == 10 && b == 20 {
        println!("AND is ok.")
    }
    // 論理和
    if a == 0 || b == 20 {
        println!("OR id ok.")
    }

    // 関数の戻り値を参照する
    // 戻り値がbool型の関数を条件式にしたif分の例
    let a = 10;
    let b = 20;
    // ifで値を返す
    let x = if a < b {1} else {0};
    println!("x is {}", x);
    // if文に続くブロックは同じ型で返さないといけない
    let x = if a < b {1} else {"hello"};
    println!("x is {}", x);

    // 繰り返し
    // ベクターを使って繰り返し処理をするfor文の例
    let v = vec![10,20,30,40,50];
    // 全ての要素を繰り返す
    println!("v is");
    for i in &v {
        print!("{} ", i);
    }
    println!("");

    // 変数xに代入するコード
    let v = vec![10,20,30,40,50];
    println!("v is ");
    for i in &v {
        print!("{}", i);
        let x : i32 = i;  // ベクターを参照しているのでコンパイルエラーになる
    }
    println!("");

    // 参照外しを使う
    println!("v is ");
    for i in &v {
        print!("{}", i);
        let x : i32 = *i; // 参照外し
    }
    println!("");

    // イテレーターを使ったfor文の例
    let v = vec![10,20,30,40,50];
    // イテレーターを使う
    print!("v is ");
    for i in v.iter() {
        print!("{} ", i);
    }
    println!("");

    // インデックス付きで繰り返すfor文の例
    let v = vec![10,20,30,40,50];
    print!("v is ");
    // インデックス付きで繰り返す
    print!("v is ");
    for (i, x) in v.iter().enumerate() {
        print!("{}:{}", i, x);
    }
    println!("");

    // for分で繰り返し数を指定する
    print!("FOR is ");
    for i in 0..10 {
        print!("{}", i);
    }
    println!("");

    // 繰り返しを途中で止める
    print!("FOR is ");
    for i in 1..10 {
        if i == 5 {
            break;
        }
        print!("{}", i);
    }
    println!("");

    // continue文を使った例
    print!("FOR is ");
    for i in 0..10 {
        if i % 2 == 0 {
            continue;
        }
        print!("{}", i);
    }
    println!("");

    // while文を使った例
    print!("WHILE is ");
    let mut i = 0;
    while i < 10 {
        print!("{}", i);
        i += 2;
    }
    println!("");

    // loop文で無限に繰り替えす
    // loopを使う
    print!("LOOP is ");
    let mut i = 0;
    loop {
        if i >=10 {
            break;
        }
        print!("{}", i);
        i += 1;
    }
    println!("");

    // パターンマッチ
    // enumで列挙型を定義する
    let lang = LANG::JAPANESE;
    println!("lang is {}", lang as i32);
    // 通常だと lang is 0 が戻ってくる

    // デバッグ出力用のアトリビュートを指定して列挙型を印刷
    let lang = LANG::JAPANESE;
    println!("lang is {:?}", lang);
    // lang is JAPANESE

    // 手動で番号を割り当てた場合
    let lang = LANG::JAPANESE;
    println!("lang is {}", lang as i32);
    // lang is 81 が戻ってくる

    // 列挙型でmatchを使う
    let lang = LANG::JAPANESE;
    // 全ての候補を列挙する =>のことをアームという
    let m = match lang {
        LANG::JAPANESE => "日本語",
        LANG::ENGLISH => "英語",
        LANG::CHINESE => "中国語",
        LANG::FRANCH => "フランス語",
    };
    println!("lang is {}", m);

    // matchに_「その他」を使った例
    let lang = LANG::ENGLISH;
    let m = match lang {
        LANG::JAPANESE => "日本語",
        _ => "日本語以外"
    };
    println!("lang is {}", m);

    // コンパイルエラーになるパターン
    let lang = LANG::CHINESE;
    // 全ての候補を列挙する =>のことをアームという
    let m = match lang {
        LANG::JAPANESE => "日本語",
        LANG::ENGLISH => "英語",
        LANG::CHINESE => "中国語",
        // LANG::FRANCHが無いよと怒られる
    };
    println!("lang is {}", m);

    // オプション型を使った例
    let x = Some(10);
    let v = match x {
        Some(i) => i,
        None => -1,
    };
    println!("v is {}", v);
    // Noneの場合
    let x = None;
    let v = match x {
        Some(i) => i,
        None => -1,
    };
    println!("v is {}", i);

    // matchをif letで置き換える
    let x = Some(10);
    match x {
        Some(i) => println!("i is {}", i),
        _ => (),
    };
    if let Some(i) = x {
        println!("i is {}", i);
    }

    // enum以外でmatchを使う
    let x = 3;
    let m = match x {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "other", // これが無いとコンパイルエラーになる
    };
    println!("m is {}", m);

    // 数値の範囲を指定した例
    let x = 5;
    let m = match x {
        0..=5 => "5以下",
        6..=10 => "6以上10以下",
        _ => "10より大きい",
    };
    println!("m is {}", m);

    // 文字列のmatch
    let x = 'C';
    let m = match x {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        _ => -1,
    };
    println!("m is {}", m);

    // 文字列をmatchさせた場合完全一致のみ動く
    let x = "three";
    let m = match x {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        _ => -1,
    };
    println!("m is {}", m);

    // アームの型も揃える必要がある
    let m = match x {
        "one" => 1,
        2 => 2,  // コンパイルエラー
        "three" => 3,
        _ => -1,
    };
    println!("m is {}", m);
}    


#[derive(Debug)] // debugのアトリビュート
enum LANG {
    // 自動採番される
    // JAPANESE,
    // ENGLISH,
    // CHINESE,
    // FRANCH,

    // 国番号を採番してみる
    JAPANESE = 81,
    ENGLISH = 44,
    CHINESE = 86,
    FRANCH = 33,
}

// オプション型
// Someが何かしらの値をもち, Noneが何も示していないという意味になる
enum Option<T> {
    Some(T),
    None,
}
