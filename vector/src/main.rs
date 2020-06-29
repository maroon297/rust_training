fn main() {
    //ベクタ
    //空のVectorを作成する際はジェネリクスで型を指定する
    let v: Vec<i32> = Vec::new();

    //マクロを使用して初期値を入れる。型推論するのでジェネリクスの指定は必要ない。
    let v2 = vec![1,2,3];

    //ベクタ更新 あとから更新する場合はmutをつける。
    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    //ベクタがドロップされたら、要素もドロップされる
    {
        let v4 = vec![1,2,3,4];
        //要素へのアクセスは2種類
        //&は参照 所有権はv4から移動しない。
        //thirdはv4の一部を参照する
        let third: &i32 = &v4[2];
        let third: Option<&i32> = v4.get(2);
        //範囲外を指定した場合の挙動が違う
        //let does_not_exist = &v4[100]; //panic!
        //let does_not_exist = &v4.get(100); //None
        //参照がある状態でpushをするとコンパイルエラー        
    }

    //ベクタの値を走査する
    let v5 = vec![100, 32, 57];
    for i in &v5 {
        println!("{}", i);
    }

    //各値の変更
    let mut v6 = vec![100, 32, 57];
    for i in &mut v6 {
        //可変参照(&mut)が参照している値を変更するには参照外し演算子(*)を使用する
        *i += 50;
        println!("{}",i);
    }

    //異なる型を入れたい場合はenumを使う
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec! [
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    
}
