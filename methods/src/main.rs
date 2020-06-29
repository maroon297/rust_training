/*
メソッド記法
メソッドは関数と似ているが、構造体の文脈、あるいはenum,トレイトオブジェクトの文脈で定義される。
*/

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//実装(implementation)ブロック
impl Rectangle {
    //メソッド
    //最初の引数は必ず&selfとなる。
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other:&Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    //引数に&selfがないのはメソッドではない。関連関数。
    //新規インスタンスを返すコンストラクタとしてよく利用される。
    //squareは1辺の長さを指定して正方形を作成する
    //呼び出しには::記法を使用する
    fn square(size : u32) -> Rectangle {
        Rectangle {width : size, height : size}
    }
}

fn main() {
    let rect1 = Rectangle { width : 30, height : 50};
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
    let rect4 = Rectangle::square(44);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()//呼び出しの際にselfの引数は必要ない
    );
    println!("Can rect1 hold rect2? {}",rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}",rect1.can_hold(&rect3));
    println!("Can rect3 hold rect4? {}",rect3.can_hold(&rect4));
}
