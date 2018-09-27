fn main() {
    println!("4.3 Copy");

    // 固定長の配列はCopy型
    let a = ["hoge", "foo"];
    let a_2 = a;
    println!("{:?}", a);

    // 要素がCopy型ではない場合はCopyできないので、コンパイルエラー
    let b = ["hoge".to_string(), "foo".to_string()];
    // let b_2 = b;
    println!("{:?}", b);
}
