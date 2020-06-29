/*
構造体
データを扱うための仕組み
メソッドは無い
*/

fn main() {
    let user1 = User {
        email: String::from("nirvana297@gmail.com"),
        username: String::from("yosuke297"),
        sign_in_count: 0,
        active: true,
    };


    //構造体更新記法
    //他のインスタンスから値を持ってきたい場合、「..変数名」と記載すれば引用してくれる
    let user2 = User {
        email: String::from("another@example.com"),
        username:  String::from("anotherusername567"),
        ..user1
    };
}

//構造体
//JAVAのPOJO的な。でもメソッドは無い。
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

//フィールド初期化省略記法
//構造体のフィールド名と仮引数名が同じ場合、フィールド名を記載するだけで値を格納できる
fn build_user (email: String, username: String) -> {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

//タプル構造体
//フィールド名を持たない。
//内容は同じだが、ColorとPointは違う型
struct Color (i32, i32, i32);
struct Point (i32, i32, i32);

//ユニット様構造体
//一切フィールドを持たない構造体
//トレイトのみ実装する場合に有効
