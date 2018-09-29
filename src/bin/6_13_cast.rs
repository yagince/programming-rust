fn main() {
    let i: u32 = 100;
    let x: usize = i as usize;

    println!("{:?} {:?}", i, x);

    let i: f32 = 0.0001;
    let x: i32 = i as i32; // 整数では表せないので、０になる
    let y: f32 = x as f32; // xは0になっているので、ここでfloatにキャストすると0.0

    println!("{:?} {:?} {:?}", i, x, y);
}
