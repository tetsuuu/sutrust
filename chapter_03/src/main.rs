fn main() {
    // 浮動小数点を扱う
    let x = 100.234;
    println!("x is {}", x );
    let x : f64 = 100.234;
    println!("x is {}", x );
    // 論理値型を使う
    let f = true;
    println!("f is {}", f );
    // 文字列型の使用
    let a = 'A';
    let c = 'あ';
    let dog = '🐶';
    let cat : char = '🐱';
    // println!の使用例
    println!("{}", a );
    println!("{}", c );
    println!("{} and {}", dog, cat);
    // String型を利用したprintln!の利用
    let s = String::from("Hello Rust world.");
    println!("{}", s );
    // 文字列の連結
    let s1 = String::from("Hello");
    let s2 = String::from("Rust");
    let s3 = String::from("world.");
    let s = s1 + " " + &s2 + " " + &s3;
    println!("{}", s );
    // formatマクロによる文字列の連結
    let ss1 = String::from("Hello");
    let ss2 = String::from("Rust");
    let ss3 = String::from("world.");
    let ss = format!("{} {} {}", ss1, ss2, ss3);
    println!("{}", ss );
    // 複合型(タプル型)
    let t = ("hogehoge", 30 );
    println!("name is {} age {}", t.0, t.1 );
    // 変数を入れたタプル型
    let name = "hogehoge";
    let age = 30;
    let tt = (name, age);
    println!("name is {} age {}", tt.0, tt.1 );
    // 配列型
    let ar = ["春", "夏", "秋", "冬"];
    println!("最初の季節 {}", ar[0]);
    println!("最後の季節 {}", ar[3]);
    // エラー例
    // let are = ["春", "夏", "秋", "冬"];
    // println!("無効な季節 {}", are[10]);
    // let are2 = ["春", "夏", "秋", "冬"];
    // let i = 10;
    // println!("無効な季節", are2[i]);
    let x2 = 100;
    let y2 = x;
    println!("x2 is {}", x2);
    println!("y2 is {}", y2);
    
    // moveエラー例
    // let xs = String::from("Hello world");
    // let ys = xs;
    // println!("xs is {}", xs);
    // println!("ys is {}", ys);
    
    // 借用エラー例
    // let xb = String::from("Hello world");
    // let len = string_lengs(xb);
    // println!("len is {}", len);
    // println!("x is {}", x);
    
    // 借用正解
    let xb2 = String::from("Hello");
    let len2 = string_lengs(&xb2);
    println!("len is {}", len2);
    println!("xb2 is {}", xb2);

    // 束縛エラー例
    // let xim = 100;
    // println!("x is {}", xim);
    // xim = 200;
    // println!("x is {}", xim);

    // mutを利用して書き換える
    let mut xim = 100;
    println!("xim is {}", xim);
    xim = 200;
    println!("xim is {}", xim);

    // シャドーイング
    let xsh = 100;
    println!("xsh is {}", xsh);
    let xsh = 200;
    println!("xsh is {}", xsh);

    // ブロック単位
    let xbl = 100;
    println!("xbl is {}", xbl);
    {
        let xbl = 200;
        println!("xbl id {}", xbl);
    }
    println!("xbl id {}", xbl);

    // 関数の呼び出し
    let ans = add_two(10, 20);
    println!("ans is {}", ans);
    let ans = add_one(30);
    println!("ans is {}", ans);

    // 構造体の利用
    let a = Sample::new(10);
    let ans = a.inc();
    println!("ans is {}", ans);
    let ans = a.add(10);
    println!("ans is {}", ans);

    // クロージャー単位
    let num = 10;
    let add_one = |x| {num + x};
    let add_two = |x, y| {x + y};

    let ans = add_one(1);
    println!("ans is {}", ans);
    let ans = add_two(10, 20);
    println!("ans is {}", ans);
}

fn string_lengs(s: &String) -> usize {
    let length = s.len();
    length
}

// エラーfunction
// fn test(x: i32) -> i32 {
//     if x < 0 {
//         let ans = 0;
//     }
//     if x > 100 {
//         let ans = 100;
//     }
//     ans
// }

// 正しいfunction
fn test(x: i32) -> i32 {
    let mut ans = x;
    if x < 0 {
        ans = 0;
    }
    if x > 100 {
        ans = 100;
    }
    ans
}
// mutを使わない場合
fn test_im(x: i32) -> i32 {
    if x < 0 {
        0
    } else if x > 100 {
        100
    } else {
        x
    }
}
//上だと複雑になった場合わかりにくいので下記でもかける
fn test_mi(x: i32) -> i32 {
    let ans = if x < 0 {
        0
    } else if x > 100 {
        100
    } else {
        x
    };
    ans
}

// 関数のスコープ
fn add_two(x: i32, y: i32) -> i32 {
    x + y
}
fn add_one(x: i32) -> i32 {
    x + 1
}

// 構造体のスコープ
struct Sample {
    x: i32,
}
impl Sample {
    fn new(x: i32) -> Sample {
        Sample {x: x}
    }
    fn inc(&self) -> i32 {
        self.x + 1
    }
    fn add(&self, x: i32) -> i32 {
        self.x + x
    }
}

// 関数は関数外の変数を参照できない
// fn test_err() {
//     let num = 10;
//     fn add_one(x: i32) -> i32 {
//         // 関数外の変数は参照できない
//         num + x
//     }
//     let ans = add_one(1);
//     println!("ans is {}", ans);
// }
