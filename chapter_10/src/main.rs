fn main() {
    // 所有権
    let x = "何か";
    let y = &x; // 参照
    let y = demo(&x); // 参照を使って関数に値を渡す
    let y = x; // 参照ではなく所有権の移動(move)
    let y = demo(x); // この時点で変数xは空になっている

    // 所有権を借用
    let a = Person { name: "hoge", age: 50,};
    print_a(&a); // &aで変数aを参照させている
    // print_a: a is Person { name: "hoge", age: 50,};
    println!("main: a is {:?}", a); // main関数に変数が残っていることがわかる
    // main: a is Person { name: "hoge", age: 50,};

    // 所有権の移動
    move_a(a);
    // println!("main: a is {:?}", a); // 所有権が移動しているのでエラーになる

    let mut a = Person { name: "fuga", age: 50, };
    println!("a is {:?}", a);
    add_age( &mut a );
    println!("a is {:?}", a);

    // 可変な変数を参照する
    let mut a = Person { name: "hoga", age: 50, };
    let mut x = &mut a;
    println!("x is {:?}", x);
    add_age( &mut x );
    println!("x is {:?}", x);

    // 数値のコピー例
    let a = 100;
    println!("a is {}", a);
    // 数値の場合は x に値が copy されている
    let x = a;
    println!("x is {}", x);
    // 所有権は a のままなので copy できる
    let y = a;
    println!("y is {}", y);

    // タプルも同様に copy が可能
    let a = (100,"hoge");
    println!("a is {:?}", a);
    // タプルの場合は x に値が copy されている
    let x = a;
    println!("x is {:?}", x);
    // 所有権は a のままなのでさらに copy できる
    let y = a;
    println!("y is {:?}", y);

    // ベクターの場合
    let a = vec!["one","two","three"];
    println!("a[] is {:?}", a);
    // ベクターの場合は x に所有権が移動される
    let x = a;
    println!("x is {:?}", x);
    // 所有権が移動しているのでエラーになる
    // let y = a;
    // println!("y is {:?}", y);

    // 文字列も所有権が移動される
    let a = String::from("fuga");
    println!("a is {}", a);
    let x = a;
    println!("x is {}", x);
    // 同様に再度の移動はできないのでエラーになる
    // let y = a;
    // println!("y is {}", y);

    // 複数の変数で所有権を奪い合う場合
    // クソコード
    let a = Person { name: "kuso", age: -1 };
    println!("a is {:?}", a);
    let x = &a;
    println!("変数xが借用する");
    println!("x is {:?}", x);
    println!("a is {:?}", a);
    // 変数yに所有権を移す
    let y = a;
    println!("y is {:?}", y);
    // 変数aも参照している変数xもつかえない
    // println!("a is {:?}", a);
    // println!("x is {:?}", x);

    // 変数のスコープ範囲
    // // エラーパターン
    // let x: &Person2;
    // // ブロックを開始する
    // {
    //     // 変数aにメモリ領域を割り当てる
    //     let a = Person2 {
    //         name: String::from("hoge"),
    //         age: 20,
    //     };
    //     // 変数xに参照させる
    //     x = &a;
    //     // ブロックをぬける
    //     // これは参照できない
    // }
    // 正しいブロック外への受け渡し
    let x: Person2;
    {
        let a = Person2 {
            name: String::from("fuga"),
            age: 30,
        };
        x = a;
    }
    println!("x is {:?}", x);

    let a = new_person("hoge", 32 );
    println!("a is {:?}", a);

    // 可変変数へのアクセス制限
    let mut a = Person2 {
        name: String::from("Error"),
        age: -1,
    };
    // println!("a is {:?}", a);
    // // 可変で参照する
    // let mut x = &mut a;
    // let mut y = &mut a;
    // x.age = 0;
    // y.name = String::from("dame"); // ここで別フィールドの更新が入ってしまっているためコンパイルエラーになる
    // println!("a is {:?}", a);
    // println!("x is {:?}", x);
    // println!("y is {:?}", y);

    // 正しい順番で参照してあげればとおる
    println!("a is {:?}", a);
    let mut x = &mut a;
    x.age = 0;
    println!("x is {:?}", x);
    let mut y = &mut a;
    y.name = String::from("Correct");
    println!("y is {:?}", y);

    // 借用の順序
    println!("a is {:?}", a);
    let mut x = &mut a ;
    x.name = String::from("Faild");
    x.age = 30;
    // エラーパターン
    // println!("a is {:?}", a);
    // println!("x is {:?}", x);
    // OKパターン
    println!("x is {:?}", x);
    println!("a is {:?}", a);
}

fn demo(a: &str) {
    print!("a is {}", a);
}

// 構造体の定義
#[derive(Debug)]
struct Person {
    name: &'static str,
    age: i32,
}

#[derive(Debug)]
struct Person2 {
    name: String,
    age: i32,
}

// 関数の定義
fn print_a( a: &Person) {
    println!("print_a: a is {:?}", a );
}

// moveする関数
fn move_a( a: Person ) {
    println!("move_a: a is {:?}", a);
}

// 加算するだけの関数
fn add_age( a: &mut Person ) {
    a.age += 1;
}

// 関数での変数の有効範囲
fn new_person ( name: &str, age: i32 ) -> Person2 {
// fn new_person ( name: &str, age: i32 ) -> &Person2 {
    let p = Person2 {
        name: String::from(name),
        age: age,
    };
    // &p 参照ではなく所有権を移動しないといけない
    p
}
