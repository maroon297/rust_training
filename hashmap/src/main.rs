fn main() {
    //HashMapを使うにはuseする必要がある
    //生成するための組み込みマクロも無い
    //HashMapはデータをヒープに保存する
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Yellow"),50);

    //HashMapの他の作り方
    //タプルのcollectメソッド
    //collectメソッドはいろんなコレクション型にデータをまとめ上げる
    let teams = vec![String::from("Blue"),String::from("Yellow")];
    let initial_scores = vec![10,50];
    let scores_from_taple: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    //所有権
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    //insertした時点で所有権は移る
    map.insert(field_name, field_value);

    //println!("{}",field_name); //エラー!
    //println!("{}",field_value); //エラー!

    let team_name = String::from("Blue");
    let score = scores.get(&team_name); //getの戻り値はOption
    match score {
        Some(value) => println!("{}",value),
        None => println!("None!")
    }

    //forで走査
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    //ハッシュマップの更新
    //insertを繰り返すと、値が上書き更新される
    let mut scores_update = HashMap::new();
    scores_update.insert(String::from("Blue"),10);
    scores_update.insert(String::from("Blue"),25);

    println!("{:?}",scores_update);

    //なかった場合にのみ更新する
    let mut scores_entry = HashMap::new();
    scores_entry.insert(String::from("Blue"),10);
    //or_insertは、対象のentryが存在した場合にはその可変参照を返す
    //存在しなかった場合は、引数をこのキーの新しい値として挿入して、そこへの可変参照を返す
    scores_entry.entry(String::from("Yellow")).or_insert(50);
    scores_entry.entry(String::from("Blue")).or_insert(50);
    println!("{:?}",scores_entry);

    //古い値に基づいて更新
    let text = "Hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}",map);

}
