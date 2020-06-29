fn main() {
    //pig-latin 単語の最初が子音なら、その文字にayを足して終端につける
    //母音なら、hayがたされる
    //文章を受け取って、空白で区切って単語のlistを作る
    //その後、それぞれの単語の1文字目がaeiouに一致する場合は-heyを足す
    //それ以外の場合はその文字にayを追加しておしりにつける
    println!("pig-latin!");

    use std::io;
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Faild to read line");
    let guess = guess.trim();
    let words_vec: Vec<&str> = guess.split(" ").collect();

    for word in words_vec {
        let characters : Vec<char> = word.chars().collect();
        if "aeiou".contains(characters[0]) {            
            println!("{}-hay",word);
        } else {
            let mut target = word.to_string();
            let first = target.remove(0);
            println!("{}-{}ay",target, first);
        }
    }
}
