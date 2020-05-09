fn main() {
    // 配列の定義例
    let ary = [1,2,3,4,5];
    println!("ary[0] is {}", ary[0]);
    println!("ary[4] is {}", ary[4]);
    println!("ary.len is {}", ary.len());

    // 添え字を変数nにして範囲外をアクセスする例
    let ary = [1,2,3,4,5];
    let n = 10;
    println!("ary[0] is {}", ary[n]);

    // 文字列str型配列の例
    let ary = ["one", "two", "three", "four", "five"];
    println!("ary[0] is {}", ary[0]);
    println!("ary[4] is {}", ary[4]);
    println!("ary.len is {}", ary.len());

    // 要素をu8型で定義した配列の例
    let ary: [u8; 5] = [0x10, 0x20, 0x30, 0x40, 0x50]; // 16進数で指定して符号なし8ビットで指定
    println!("ary[0] is {}", ary[0]);
    println!("ary[4] is {}", ary[4]);
    println!("ary.len is {}", ary.len());

    // 要素数が0である配列の例
    // 空の配列の場合型推論が効かないため、変数定義時に型を指定しないといけない
    let ary: [u8; 0] = [];
    println!("ary.len is {}", ary.len());

    // 配列の初期化
    // 0で初期化する
    let mut ary: [u8; 16] = [0; 16]; // 符号なし8ビットが16個配列にあり、16個とも0が入っている
    println!("ary[0] is {}", ary[0]);
    println!("ary[15] is {}", ary[15]);
    // 値を変更する
    ary[0] = 10;
    println!("ary[0] is {}", ary[0]);
    println!("ary[15] is {}", ary[15]);

    // 配列型を強制キャストする
    // u8型とu32型を相互変換するコードの例
    let a = [1u8,2u8,3u8,4u8];
    // let b = a as u32;
    unsafe {
        let b = std::mem::transmute::<[u8; 4], u32>(a);
        println!("b is {:X}", b);
    }
    // u32を4つのu8に変換する
    let a: u32 = 0x11223344;
    unsafe {  // 強制的なキャストはコンパイルエラーになるのでunsafeで囲んでコンパイルエラーを抑制している
        let b = std::mem::transmute::<u32, [u8; 4]>(a);
        println!("b[] is {:X} {:X} {:X} {:X}",
            b[0], b[1], b[2], b[3]);
    }
    // b is 40302010
    // b[] is 44 33 22 11

    // ベクター初期化の例
    let v = vec![1,2,3,4,5];
    println!("v[0] is {}", v[0]);
    println!("v[4] is {}", v[4]);
    println!("v.len is {}", v.len());

    // ベクターのgetメソッドの例
    println!("v.get(0) is {:?}", v.get(0));
    println!("v.get(4) is {:?}", v.get(4));
    println!("v.get(0) is {}", v.get(0).unwrap());
    println!("v.get(4) is {}", v.get(4).unwrap());
    // v.get(0) is Some(1) getメソッドは値をくるんだオプション型で取得される
    // v.get(4) is Some(5) オプション型で帰ってくる理由は要素数以上の添え字にアクセスした際にNoneを返すため
    // v.get(0) is 1
    // v.get(4) is 5

    // 文字列のベクターの例
    let v = ["one", "two", "three", "four", "five"]; 
    println!("v[0] is {}", v[0]);
    println!("v[4] is {}", v[4]);
    println!("v.len is {}", v.len());

    // ベクターの要素の追加・削除の例
    let mut v: Vec<i32> = Vec::new();
    println!("v.len is {}", v.len());
    // 要素を末尾に追加
    v.push(10);
    v.push(20);
    v.push(30);
    println!("v.len is {}", v.len());
    // 要素を末尾から取得
    println!("pop is {:?}", v.pop());
    println!("pop is {:?}", v.pop());
    println!("pop is {:?}", v.pop());
    println!("v.len is {}", v.len());
    // v.len is 0
    // v.len is 3
    // pop is Some(30)
    // pop is Some(20)
    // pop is Some(10)
    // v.len is 0

    // 型定義をしない形に修正
    let mut v = Vec::new();
    println!("v.len is {}", v.len());
    v.push(10); // この時点まで変数vの型は推論されない
    v.push(20);
    v.push(30);
    println!("pop is {:?}", v.pop());
    println!("pop is {:?}", v.pop());
    println!("pop is {:?}", v.pop());
    println!("v.len is {}", v.len());

    // ベクター要素の参照
    // first/lastメソッドを使った先頭/末尾の要素の参照例
    let mut v = vec![1,2,3,4,5];
    // 先頭の要素
    println!("v.first is {:?}", v.first());
    // 末尾の要素
    println!("v.last is {:?}", v.last());
    // 要素の値を取得
    println!("v.first is {}", v.first().unwrap());
    println!("v.last is {}", v.last().unwrap());
    // v.first is Some(1)
    // v.last is Some(5)
    // v.first is 1
    // v.last is 5

    // ベクターの要素を削除する(removeメソッドの利用)
    let mut v = vec![1,2,3,4,5];
    println!("v.first is {:?}", v.first());
    println!("v.remove(0) is {:?}", v.remove(0));
    println!("v.first is {:?}", v.first());
    // v.first is Some(1)
    // v.remove(0) is 1
    // v.first is Some(2)
    
    // 指定位置に要素を追加する(insertメソッド)
    println!("v.first is {:?}", v.first());
    v.insert(0, 10);
    println!("v.first is {:?}", v.first());
    // 末尾に追加
    v.insert(v.len(), 99);
    println!("v.last is {:?}", v.last());
    // v.first is Some(1)
    // v.first is Some(10)
    // v.last is Some(99)

    // 複数のベクターをつなげる
    let a = vec![1,2,3];
    let b = vec![4,5];
    let v = [a, b].concat();
    println!("v.len is {}", v.len());
    for i in v {
        print!("{}", i);
    }
    println!("");

    // 連結文字を指定してひとつながりの文字列にする例
    let v = vec!["one", "two", "three", "four", "five"];
    let x = v.join("-");
    println!("x is {}", x);
    // x is one-two-three-four-five

    // 分割文字を指定してベクターを切り分ける例
    let s = "one,two,three,four,five";
    let v = s.split(',');
    for x in v {
        print!("{}", x);
    }
    println!("");

    // ベクターをソートする例
    let mut v = vec!["one", "two", "three", "four", "five"];
    v.sort();
    let x = v.join(" ");
    println!("x is {}", x);
    // 逆順にする
    v.reverse();
    let x = v.join(" ");
    println!("x is {}", x);
    // x is five four one three two
    // x is two three ome four five

    // for文でイテレーターを使う
    let v = vec![1,2,3,4,5];
    print!("for is ");
    for i in &v {
        print!("{} ", i);
    }
    // forとイテレーターの利用
    print!("for and iter is ");
    for i in v.iter() {
        print!("{}", i);
    }
    println!("");

    // nextメソッドで次の要素のイテレーターを得る
    let v = vec![1,2,3,4,5];
    let mut p = v.iter();
    println!("p is {:?}", p);
    println!("p is {:?}", p.next()); // iterメソッドはオプション型で返す
    println!("p is {:?}", p.next());
    println!("p is {:?}", p.next());
    println!("p is {:?}", p.next());
    println!("p is {:?}", p.next());
    println!("p is {:?}", p.next()); // 6回目はnoneになる

    // loop文やwhile文でイテレーターを使う
    let v = vec![1,2,3,4,5];
    // loop文で冗長に書く
    println!("by loop");
    let mut p = v.iter();
    loop {
        let x = p.next();
        if x == None {
            break;
        }
        println!("x is {}", x.unwrap());
    }
    // while文で書く
    println!("by while");
    let mut p = v.iter();
    while let Some(x) = p.next() {
        println!("x is {}", x);
    }

    // 要素の10倍を返すプログラム例
    let v = vec![1,2,3,4,5];
    // mapを使う
    let lst = v.iter().map(|x| x * 10); // mapメソッドはクロージャーなどを利用して要素に対する加工を設定する
    for i in lst {
        println!("i is {}", i);
    }
}
