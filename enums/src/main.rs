enum IpAddr {
    //enum列挙子には値を紐付けられる。
    V4(u8, u8, u8, u8),
    //それぞれの列挙子に別の型を紐付けることも可能
    V6(String),
}

enum Message {
    Quit,
    //匿名構造体
    Move{ x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),    
}

impl Message {
    fn call(&self) {
        println!("called!!");
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin : Coin) -> u32 {
    match coin {
        //複数行の場合は{}を使用する
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }

}

fn plus_one(x : Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let home = IpAddr::V4(1,1,1,1);
    let loopback = IpAddr::V6(String::from("::1"));
    let a = Message::Write(String::from("called"));
    a.call();

    //Option<T>
    //Some or None
    let some_number = Some(5);
    let some_string = Some("option");
    let absent_number : Option<i32> = None;

    let coin = Coin::Quarter(UsState::Alabama);
    value_in_cents(coin);    

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    let test = vec![1,2,3,4,5];
}
