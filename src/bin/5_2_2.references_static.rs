fn main() {
    sub();
    unsafe {
        println!("{:?}", STASH);
    }
    println!("done");
}

fn sub() {
    let i: &i32 = &32;
    f(i); // iの生存期間はSTASHの生存期間より短いはずな気がするけど、なぜこれがコンパイル通るのか理解できてない

    println!("{:?}", i);
}

static mut STASH: &i32 = &128;
fn f(p: &'static i32) {
    unsafe {
        // 可変なstatic変数はスレッドセーフじゃないので、unsafeブロック内でないとアクセスできない（仕様）
        STASH = p;
    }
}
