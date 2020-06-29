use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

fn main() {
    /*
    panic!はプログラムは失敗のメッセージを表示し、スタックを巻き戻して掃除して終了
    Resultは、Ok or Errを含むenum
     */
    let f = match File::open("hello.txt") {
        Ok(file)  => file,
        //matchのアームにはif文が続けられる。マッチガードという。
        //マッチガードに一致する場合はそのアームの処理が実施される。
        //refはマッチガードに参照されないために必要。&の代わりにパターン内で参照を作っている?
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {                    
                    panic!("Tried to create file but there was a problem: {:?}",e);
                }
            }
        },
        Err(error) => {
            panic!("There was a problem opening the file : {:?}",error);
        }
    };

    //unwrapは、ResultがOKならその中身を、Errならpanic!を実行する。
    let f1 = File::open("hello.txt").unwrap();

    //expectは、ResultがOKならその中身を、Errならpanic!し、指定されたメッセージを出力
    let f2 = File::open("hello.txt").expect("Failed to open hello.txt");

}

//エラーの委譲
//エラーを上に上げたい場合、戻り値をResultにする
//そして、正常な値をエラー時の値をセットする
fn read_username_from_file() -> Result<String,io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

//?を使って簡潔に記載する
//Resultを返す関数でしか使用できない
//Resultがokの場合はそれを変数に格納し、Errの場合はErrorをリターンする
fn read_username_from_file_() -> Result<String,io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
