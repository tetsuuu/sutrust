fn main() {
    // ジェネリクス
    // ベクターは空のジェネリクス
    let mut v = vec![1,2,3,4,5];
    // Vet::new でベクターを初期化
    let mut v: Vec<i32> = Vec::new();
    // 型を指定してVet::new でベクターを初期化
    let mut v = Vec::<i32>::new();
    // 様々な型をまとめて扱えるベクター
    let mut v: Vec::<&str> = Vec::new();
    let mut v: Vec::<&String> = Vec::new();
    let mut v: Vec::<f64> = Vec::new();

    // ジェネリックを使わない関数の呼び出し
    let v = [10,20,30,40,50];
    print_i32(&v);
    let v = ['A','B','C','D','E'];
    print_char(&v);
    let v = ["one","two","three","four","five"];
    print_str(&v);

    // ジェネリックを使った関数の呼び出し
    let v = [10,20,30,40,50];
    print(&v);
    let v = ['A','B','C','D','E'];
    print(&v);
    let v = ["one","two","three","four","five"];
    print(&v);

    // traitにて各図形の面積を求めてみる
    let rect = Rectange {
        width: 10.0,
        height: 20.0,
    };
    let tri = Triangle {
        base: 10.0,
        height: 20.0,
    };
    let cir = Circle {
        radius: 10.0,
    };
    println!("rect area is {}", rect.calc_area());
    println!("tri area is {}", tri.calc_area());
    println!("cir area is {}", cir.calc_area());
    // expr_strを利用するパターン
    println!("rect {} {}", rect.expr_str(), rect.calc_area());
    println!("tri {} {}", tri.expr_str(), tri.calc_area());
    println!("cir {} {}", cir.expr_str(), cir.calc_area());

    // 既存の構造体に追加したメソッドを利用する
    let s = String::from("100");
    let n = s.to_i();
    println!("n is {}", n);
}

// ジェネリックを使わない関数
fn print_i32( a: &[i32] ) {
    print!("a is ");
    for i in a {
        print!("{}", i);
    }
    println!("");
}

fn print_char( a: &[char]) {
    print!("a is ");
    for i in a {
        print!("{}", i);
    }
    println!("");
}

fn print_str( a: &[&str]) {
    print!("a is ");
    for i in a {
        print!("{}", i);
    }
    println!("");
}

// ジェネリックを使った関数
fn print<T>( a: &[T]) where T: std::fmt::Debug {
    print!("a is ");
    for i in a {
       print!("{:?}", i);
    }
    println!("");
}

// トレイト
// structに一定のルールを付け加える機能
// 多言語のinterfaceに近いがさらに汎用的に使える

// 図形の構造体とその面積を求めるtrait
trait CalcArea {
    fn calc_area(&self) -> f32;
}

// デフォルトのtrait
trait ExprString {
    fn expr_str(&self) -> String {
        "幅 × 高さ = ".to_string()
    }
}

// 既存の構造体へのtraitの追加
trait ToNumber {
    fn to_i(&self) -> i32;
}

impl ToNumber for String {
    fn to_i(&self) -> i32 {
        match self.parse::<i32>() {
            Ok(n) => n,
            Err(_) => 0,
        }
    }
}

// 四角形の構造体
struct Rectange {
    width: f32,
    height: f32,
}

impl CalcArea for Rectange {
    fn calc_area(&self) -> f32 {
        self.width * self.height
    }
}

impl ExprString for Rectange {}

// 三角形の構造体
struct Triangle {
    base: f32,
    height: f32,
}

impl CalcArea for Triangle {
    fn calc_area(&self) -> f32 {
        self.base * self.height * 0.5
    }
}

impl ExprString for Triangle {
    fn expr_str(&self) -> String {
        "底辺 × 高さ ÷ 2 = ".to_string()
    }
}

// 円の構造体
struct Circle {
    radius: f32,
}

impl CalcArea for Circle {
    fn calc_area(&self) -> f32 {
        self.radius * 3.14
    }
}

impl ExprString for Circle {
    fn expr_str(&self) -> String {
        "π × 半径 × 半径 = ".to_string()
    }
}
