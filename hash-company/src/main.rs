fn main() {
    /*
    3つのコマンドを受け取る
    1. Add A to B : Bの名前をAの部署に紐付ける
    2. show A : Aの部署の名簿を表示する
    3. show : すべての部署のすべての名前を表示する
     */
    extern crate regex;
    use std::io;
    use regex::Regex;
    use std::collections::HashMap;

    let mut company_map = HashMap::new();
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Faild to read line");
        let add_re = Regex::new(r"^Add (.*) [Tt][Oo] (.*)$").unwrap();
        let show_re = Regex::new(r"^Show (.*)$").unwrap();
        let show_dep_re = Regex::new(r"^Show$").unwrap();
        if add_re.is_match(guess.trim()) {
            let caps = add_re.captures(guess.as_str()).unwrap();
            let dep = company_map.entry(caps.at(2).unwrap()).or_insert(Vec::new());
            dep.push(caps.at(1).unwrap());
        } else {
            println!("NG");
        }     
    }
}
