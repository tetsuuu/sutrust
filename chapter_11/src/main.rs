use rand::prelude::*;
use my_random::random::*;

fn main() {
    // クレートの利用
    let x: i32 = random();
    println!("x is {}", x);

    let mut rng = thread_rng();
    let y: i32 = rng.gen_range(0, 10);
    println!("y is {}", y);

    println!("Hello world");

    // 作成したライブラリの利用
    // ./my_random でcargo build --releaseを実施する
    // target/release配下にできるlibmy_random.rlibをこちらのtarget配下にもってきてCargo.tomlに追記すると使える
    let dice = Dice{max: 6};
    let x = dice.get();
    println!("x is {}", x);

    let mut data = vec![0,0,0];
    dice.fill(&mut data);
    for i in data {
        println!("i is {}", i);
    }
}
