pub trait Summary {
    //実装すべきメソッド
    fn summarize_author(&self) -> String;
    //デフォルト実装付きのメソッド
    fn summarize(&self) -> String {
        //同じトレイトで宣言されているメソッドを呼び出せる
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("author: {}",self.author)
    }
    //デフォルト実装をそのまま使う場合はオーバーライドしない
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}",self.username, self.content)
    }
    //トレイトで実装済みの動作を上書きする場合はオーバーライドする
    fn summarize_author(&self) -> String {
        format!("@{}",self.username)
    }
}

//トレイト境界を設定する
//Summaryを実装する型のみを引数に取れるように
//+を使用して複数のトレイトを指定することもできる
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

//whereで分離して書くこともできる
pub fn trait_border<T, U>(t:T,u:U) -> i32
where T: Clone, U: Summary
{
    1
}
