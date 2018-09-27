use std::process::Command;
use std::io::Result;

fn main() {
    let s = Some("hoge");

    if let Some(name) = s {
        println!("{:?}", name);
    }

    let result = Command::new("hoge").output();
    if let Err(err) = result {
        println!("{:?}", err);
    }
}
