fn main() {
    wait();
}

fn wait() -> ! { // `!` は返ってこないという意味の特別な型
    let mut i = 0;
    loop {
        println!("{:?}", i);
        i += 1;
    }
}

// ↓は実際はi32が返らないことはありえないが、コンパイルは通らない
// use std::process;
// fn wait_while() -> i32 {
//     while true {
//         process::exit(0);
//     }
// }
