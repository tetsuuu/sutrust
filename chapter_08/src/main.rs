fn main() {
    // 構造体のインスタンス化(実際にメモ領域を確保すること)
    let mut pa = Person {
        id: 100,
        name : String::from("hoge"),
        age: 50,
        addr: String::from("Tokyo"),
    };
    // 構造体のフィールドを参照
    println!("{}: {} ({}) in {}",
        pa.id, pa.name, pa.age, pa.addr);
    pa.age += 1;
    println!("{}: {} ({}) in {}",
        pa.id, pa.name, pa.age, pa.addr);
    pa.addr = String::from("Osaka");
    println!("{}: {} ({}) in {}",
        pa.id, pa.name, pa.age, pa.addr);
    
    // 関数の呼び出し
    print_person(&pa);
    // 100: hoge (51) Osaka

    // 構造体のフィールドを変更
    let mut pa = Person {
        id: 100,
        name: String::from("fuga"),
        age: 50,
        addr: String::from("Tokyo"),
    };
    // 関数の呼び出し
    print_person(&pa);
    // 構造体を変更する関数の呼び出し
    add_age(&mut pa);
    print_person(&pa);
    // 100 hoge (50) in Tokyo
    // 100 hoge (51) in Tokyo

    // Person構造体を作成
    let pa2 = new_person(200, "fuga");
    println!("{}: {} ({}) in {}",
        pa2.id, pa2.name, pa2.age, pa2.addr);
    // 200 fuga (-1) in Unkwown

    // ベクターで構造体を扱う
    let pa = new_person(100, "hoge");
    let pa2 = new_person(200, "fuga");
    let mut people = vec![pa, pa2];
    people.push(new_person(200, "hoga")); // new_person関数を使って新たな構造体をベクターに格納している
    people.push(new_person(100, "fuge"));

    for p in &people {
        print_person( p ); // &p でもOK
    }
    // 100: hoge (-1) in Unknown
    // 200: fuga (-1) in Unknown
    // 200: fuge (-1) in Unknown
    // 200: hoga (-1) in Unknown

    // フィールド名を使わない構造体の例
    struct Color(f32,f32,f32);
    let yellow = Color(1.0,1.0,0.0);
    println!("R:{::2} G:{::2} B:{::2}",
        yellow.0, yellow.1, yellow.2);
    // R:1.00 G:1.00 B:0.00

    // 構造体のサイズ
    println!("A size_of is {}", std::mem::size_of::<A>());  // std::mem::size_of::<型>()で構造体が必要とするメモリ領域がわかる
    println!("B size_of is {}", std::mem::size_of::<B>());
    println!("C size_of is {}", std::mem::size_of::<C>());
    println!("D size_of is {}", std::mem::size_of::<D>());
    println!("E size_of is {}", std::mem::size_of::<E>());
    println!("F size_of is {}", std::mem::size_of::<F>());
    // A size_of is 4   i32(4バイト)が1つなので4
    // B size_of is 12  i32(4バイト)が3つなので32
    // C size_of is 32  各文字列やベクターが確保するポインタごとに違う
    // D size_of is 24  各文字列やベクターが確保するポインタごとに違う
    // E size_of is 32  各文字列やベクターが確保するポインタごとに違う
    // F size_of is 104 i32(4バイト)が1つとu8(1バイト)が100なので104

    // メソッドの利用
    let pa = Person {
        id: 1,
        name: String::from("hoge"),
        age: 50,
        addr: String::from("Tokyo"),
    };
    pa.print();

    // 引数のあるメソッドの利用
    pa.print_t(true);
    pa.print_t(false);

    // 戻り値を持つメソッドの利用
    let s = pa.to_str();
    println!("s is {}", s);

    // フィールドを変更するメソッドの利用
    pa.print();
    pa.add_age(1);
    pa.print();

    // newメソッドを使って新しいPersonを作成する
    let mut people = Vec::<Person>::new();
    people.push(Person::new("hoge", 50, "Tokyo"));
    people.push(Person::new("fuga", 30, "Osaka"));
    people.push(Person::new("hoga", -1, "unknown"));
    people.push(Person::new("fuge", -1, "unknown"));
    for p in &people {
        p.print();
    }

    // str::parseの戻り値を探る
    let r = "100".parse::<i32>(); // 100という文字列をi32型に変換する
    match r {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("error: {:?}", e),
    }
    // n is 100

    // 数値に変換できない文字列の変換
    let r = "xxx".parse::<i32>();
    match r {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("error: {:?}", e),
    }
    // error: ParseIntError { kind: InvalidDigit }

    // unwrapメソッドの利用例(ResultのOKの値を取得できる)
    let n = "100".parse::<i32>().unwrap();
    println!("n is {}", n);
    // n is 100
    let n = "xxx".parse::<i32>().unwrap();
    println!("n is {}", n);
    // この場合panicを起こしてしまう

    // Result型の関数を呼び出す
    let n: i32 = half_number("100"); // xxxにするとpanicを起こす
    println!("n is {}", n);
    // n is 50

    // Result型を返す関数を呼び出す
    match half_number_res("100") {
        Ok(n) => println!("Ok: {}", n),
        Err(err) => println!("Error: {:?}", err),
    }
    // Ok: 50
    match half_number_res("xxx") {
        Ok(n) => println!("Ok: {}", n),
        Err(err) => println!("Error: {:?}", err),
    }
    // error: ParseIntError { kind: InvalidDigit }

    match half_number2("100") {
        Ok(n) => println!("Ok: {}", n),
        Err(err) => println!("Error: {:?}", err),
    }

    // コンピネーターでmatchを減らした関数を呼び出す
    match half_number3("100") {
        Ok(n) => println!("Ok: {}", n),
        Err(err) => println!("Error: {:?}", err),
    }
    match half_number3("xxx") {
        Ok(n) => println!("Ok: {}", n),
        Err(err) => println!("Error: {:?}", err),
    }

    // エラーを委譲する関数を呼び出す
    match half_number4("100") {
        Ok(n) => println!("Ok: {}", n),
        Err(err) => println!("Error: {:?}", err),
    }

    // 独自のエラーを返す関数を呼び出す
    match half_number5("100") {
        Ok(n) => println!("Ok: {}", n),
        Err(err) => println!("Error: {:?}", err),
    }
    match half_number5("xxx") {
        Ok(n) => println!("Ok: {}", n),
        Err(err) => println!("Error: {:?}", err),
    }
    // Ok: 50
    // Error: "実行エラー : これは数値ではありません"

    // exceptメソッドを使う
    let n = "100".parse::<i32>()
        .expect("これは数値ではありません");
    println!("n is {}", n);
    let n = "xxx".parse::<i32>()
        .expect("これは数値ではありません");
    println!("n is {}", n);
    // n is 100
    // panicを起こす

}

// 構造体の定義
// 構造体とはデータの固まりで、フィールドに名前をつけて保存できる。
// フィールドの無い構造体を作ることも可能
struct Person {
    id: i32,
    name: String,
    age: i32,
    addr: String, // ケツカンマ無くてもコンパイル通るが慣例的につける
}

// 一行でも書くことができる
struct PersonOneLine {id: i32, name: String, age: i32, addr: String }

// 構造体を引数にとる関数
fn print_person( pa: &Person) {
    println!("{}: {} ({}) in {}",
        pa.id, pa.name, pa.age, pa.addr);
}

// フィールドが可変な構造体を引数にとる
fn add_age( pa: &mut Person) {
    pa.age += 1;
}

// 構造体を戻り値にする関数
fn new_person(id: i32, name: &str) -> Person {
    let pa = Person {
        id: id,
        name: name.to_string(),
        age: -1,
        addr: String::from("Unknown"),
    };
    pa
}

// 構造体のサイズを調べる
struct A {
    id: i32,
}

struct B {
    id1: i32,
    id2: i32,
    id3: i32,
}

struct C {
    id: i32,
    name: String,
}

struct D {
    id: i32,
    name: &'static str,
}

struct E {
    id: i32,
    v: Vec<u8>,
}

struct F {
    id: i32,
    v: [u8; 100],
}

// メソッドを定義
impl Person {
    fn print(&self) {
        println!("{}: {} ({}) in {}",
            self.id, self.name, self.age, self.addr);
    }
}

// 引数のあるメソッドを定義
impl Person {
    fn print_t(&self, private: bool) {
        if private == true {
            println!("{}: {}",
             self.id, self.name );
        } else {
            println!("{}: {} ({}) in {}",
             self.id, self.name, self.age, self.addr);
        }
    }
}

// 戻り値を持つメソッドを定義
impl Person {
    fn to_str(&self) -> String {
        let s = format!("{}: {} ({}) in {}",
            self.id, self.name, self.age, self.addr);
        s // 式として書いているので;は不要
    }
}

// フィールドを変更するメソッド
impl Person {
    fn add_age(&mut self, n: i32) { // &mut を追加して可変にする必要がある
        self.age += n;
    }
}

// newメソッドを定義する
// JavaやC++にあるインスタンスを作成するnew演算子がRustには無いので自前で作っちゃう
static mut PERSON_ID: i32 = 0;
impl Person {
    // 新しいPersonを作る
    fn new(name: &str, age: i32, addr: &str) -> Person {
        // グローバル変数を使い新しいIDを作成
        let id = unsafe {
            PERSON_ID += 1;
            PERSON_ID
        };
        //Personを返す
        Person {
            id: id,
            name: name.to_string(),
            age: age,
            addr: addr.to_string(),
        }
    }
}

// Result型の定義
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// Result型を返す関数
fn half_number(s: &str) -> i32 {
    s.parse::<i32>().unwrap() / 2
}

use std::num::ParseIntError;

// 完全なResult型を返す関数
fn half_number_res(s: &str) -> Result<i32, ParseIntError> {
    match s.parse::<i32>() {
        Ok(n) => Ok(n / 2),
        Err(err) => Err(err),
    }
}

// Result型のエイリアスを作る
type MyResult<T> = std::result::Result<T, ParseIntError>;
// Result<T>を返す修正版
fn half_number2(s: &str) -> MyResult<i32> {
    // ここでResultをうける
    match s.parse::<i32>() {
        Ok(n) => Ok(n / 2),
        Err(err) => Err(err),
    }
}

// コンピネーターでmatch文を減らす
fn half_number3(s: &str) -> Result<i32, ParseIntError> {
    s.parse::<i32>().map(|n| n / 2)
}

// エラー委譲でmatch文を減らす
fn half_number4(s: &str) -> Result<i32, ParseIntError> {
    let n = s.parse::<i32>()?;
    Ok(n / 2)
}

// 独自のResult型を返す
fn half_number5(s: &str) -> Result<i32, &str> {
    match s.parse::<i32>() {
        Ok(n) => Ok(n / 2),
        Err(err) => Err("実行エラー : これは数値ではありません"),
    }
}

    // main関数でResultを返す
fn _main() -> Result<(), Box<dyn std::error::Error>> {    
    // ファイルを一気に読み込む
    let path = "sample.txt";
    let data = std::fs::read_to_string(path)?;
    println!("data is {}", data);
    Ok(())
    // data is hello rust world
}

fn e_main() -> Result<(), Box<dyn std::error::Error>> {    
    // ファイルを一気に読み込む
    let path = "unknown.txt";
    let data = std::fs::read_to_string(path)?;
    println!("data is {}", data);
    Ok(())
    // Error: 0s { code: 2, kind: NotFound, message: "No such file or directory" }
}
