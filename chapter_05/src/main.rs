fn main() {
    // Rustにおける文
    // 並列で書く
    let a = 10; let b = 20; println!("a is {}, b is {}", a, b);
    // 改行を入れるパターン
    let a
      = 10;
    let b
      = 20;
    println!("a is {}, b is {}",
      a, b);

      // 式を使った例
      let a = 10 + 20;
      println!("a is {}", a);
      // ブロックの場合も値を返すので式にできる
      let a = {10 + 20};
      println!("a is {}", a);

      // 関数として式を使った場合
      let a = add(10, 20);
      println!("a is {}", a);

      // if分の条件式を設定した例
      let a = 10;
      if a > 0 { // bool型としてtrueが返ってくる
          println!("a is {}", a);
      }

      // if分の条件式にbool型を戻り値にもつ関数を設定した例
      let a = 10;
      if plus(a) {
          println!("plus(a) is {}", a);
      }

      // 一般的な四則演算子
      let a = 20 + 10;
      println!("a is {}", a);
      let a = 20 - 10;
      println!("a is {}", a);
      let a = 20 * 10;
      println!("a is {}", a);
      let a = 20 / 10;
      println!("a is {}", a);
      let a = 20 % 3;
      println!("a is {}", a);

      // 整数と実数の割り算の違い
      let a = 10;
      let b = 3;
      let ans = a / b;
      println!("a / b is {}", ans);
      // a / b is 3
      let a = 10.0;
      let b = 3.0;
      let ans = a / b;
      println!("a / b is {}", ans);
      // a / b is 3.3333333333335

      // 代入演算子の使い方
      // 加算しながら代入ができる
      let mut a = 10;
      a += 20;
      // a = a + 20; と同じ
      println!("a is {}", a)
      // 繰り返し処理で利用する
      let mut sum = 0;
      for i in 0..10 {
          sum += i;
      }
      println!("sum is {}", sum);

      // ビット演算子の使い方
      let a : u8 = 0b1111;
      let b : u8 = 0b0011;
      println!("a & b is {:04b}", a & b);
      prtinln!("a | b is {:04b}", a | b);

      // シフト演算子
      let a : u8 =0x02;
      println!("a << 1 is {}", a << 1);
      println!("a >> 1 is {}", a >> 1);

      // 論理演算子
      let a = true;
      let b = false;
      println!("a && b is {}", a && b);
      println!("a || b is {}", a || b);
      // a && b is false
      // a || b is true

      // 単項演算子の使い方
      let a = true;
      let b = !a;
      println!("a is {}, b is {}", a, b);

      // 引数のない関数
      no_param();

      // 引数のある関数
      one_param(10);
      two_param(10, 20);

      // 戻り値のある関数の例
      let ret = two_param_and_return(10, 20);
      println!("ret is {}", ret);

      // 文字列をうけとる関数の例
      str_param("rust");

      // 関数の中が複雑になる例
      let mut s = String::new();
      str_param_complex(&mut s);
      ptintln!("s is {}", s);

      // 文字列を返す関数の例
      let ret = str_param_and_return("rust");
      println!("ret is {}", ret);

      // ベクターを受け取る関数の例
      let v = vec![1,2,3,4,5];
      let sum = vec_param(&v);
      println!("sum is {}", sum);

      // ベクターを返す関数の例
      let v = vec_return(10);
      for i in v {
          print!("{}", i);
      }
      println!("");

      // ベクターの中身を変更する関数の例
      let mut v = vec![1,2,3,4,5];
      vec_change(&mut v);
      for i in v {
          print!("{}", i);
      }
      println!("");
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn plus(x: i32) -> bool {
    x > 0
}

fn no_param() {
    println!("called no_param");
}

// 引数が1つの関数
fn one_param(x: i32) {
    println!("called one_param, x is {}", x);
}
// 引数が2つの関数
fn two_param(x: i32, y: i32) {
    println!("called two_param, {} and {}", x, y);
}

// 戻り値のある関数
fn two_param_and_return(x: i32, y: i32) -> i32 {
    println!("called two_param_and_return, {} and {}", x, y);
    x + y
}

fn str_param(s: &str) {
    println!("called str_param, s is {}", s);
}

// 関数内で引数sの値を変更する (&str型は変更不可のためエラーになる)
fn str_param_err(s: &str) {
    s = "change rust";
    println!("called str_param_err, s is {}", s);
}

// 文字列を受け取る関数
fn str_param_complex(s: &mut String) {
    *s = String::from("hello");
}

// 文字列を返す関数
fn str_param_and_return(s: &str) -> String {
    println!("called str_param_and_return, s is {}", s);
    let ret = format!("hello {} world.", s);
    ret
}

// ベクターを受け取る関数
fn vec_param(v: &Vec<i32>) -> i32 {
    println!("called vec_param");
    let mut sum = 0;
    for i in v {
        sum += i;
    }
    sum
}

// ベクターを返す関数
fn vec_return(max: i32) -> Vec<i32> {
    println!("called vec_return");
    let mut v = Vec::new();
    for i in 0..max {
        v.push(i);
    }
    v
}

// ベクターの中身を変更する関数
fn vec_change(v: &mut Vec<i32>) {
    println!("called vec_change");
    for i in v {
        *i = *i * 10;
    }
}
