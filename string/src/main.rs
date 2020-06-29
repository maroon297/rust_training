/*
Rustにはcoreとしては文字列型は1種類しか文字列型は存在しない。
文字列スライス str で、通常は借用された形(&str)
Stringは標準ライブラリとして提供される
UTF8でエンコードされる。
*/

fn main() {
    //String型
    let mut s = String::new();

    let data = "initial contents";
    //to_stringメソッドはDisplayトレイトがある型なら何でも使える。
    let s1 = data.to_string();

    let s2 = "initial contents".to_string();

    //String::fromで初期化する
    let s3 = String::from("initial contents");

    //push_strは所有権は奪わない。借用するだけ。
    let mut s4 = String::from("foo");
    s4.push_str("bar");

    //文字列の結合
    //S52が参照を使用した理由は、+がaddを使用しているため。
    //addは、&strを受け取る。Stringは&strしか追加することはできない。
    //&s52は&Stringとなっているが、コンパイル時に&Stringを&strに型強制する。
    let s51 = String::from("Hello, ");
    let s52 = String::from("world!");
    let s53 = s51 + &s52; //s51やムーブされてもう使用できない
    //文字列結合はformat!を使用すると楽か
    let s61 = String::from("Hello, ");
    let s62 = String::from("world!");
    let s63 = format!("{}{}",s61,s62);
    println!("{}",s63);
}
