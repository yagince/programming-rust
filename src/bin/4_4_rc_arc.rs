use std::rc::Rc;

fn main() {
    println!("4.4 Rc & Arc");

    // RCは Reference Countつまり参照カウント
    let mut s = Rc::new("hoge".to_string());
    // let x = s; // move してしまうと以降使えないのは同じ
    let t = s.clone(); // cloneすると参照カウントがインクリメントされるが実際はコピーされない
    let u = s.clone();
    // 最後のRCがDropされるとStringもDropされる

    assert!(s.contains("ho")); // 普通にStringのメソッドが使える
    // s.push_str("hoge"); // sがmutなのに、書き換えられない (cannot borrow as mutable)

    // これはできる普通に
    let mut str = "hoge".to_string();
    str.push_str("foo");
    println!("{}", str);
}
