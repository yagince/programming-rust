fn main() {
    let strings = vec![
        "hoge0".to_string(),
        "hoge1".to_string(),
        "hoge2".to_string(),
        "hoge3".to_string(),
        "hoge4".to_string(),
    ];

    // for s in strings { // moved
    //     println!("{:?}", s);
    // }

    for s in &strings { // &Vec<String> -> s is &String
        println!("{:?}, {:p}", s, s);
    }

    for s in strings.iter() { // &Vec<String> -> s is &String
        println!("{:?}, {:p}", s, s);
    }

    println!("{:?}", strings);
}
