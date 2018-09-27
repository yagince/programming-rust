fn main() {
    let v = vec![4, 8, 9, 27, 34, 10];
    // let r = &v;
    // let aside = v; // borrowしているので、moveできない
    // r[0];

    {
        let r = &v;
        r[0];
    } // ここでｒがドロップされるので、この後はmoveできる
    let aside = v;


    let mut wave = Vec::new();
    let head = vec![0.0, 0.1];
    let tail = [0.0, -1.0];

    extend(&mut wave, &head);
    extend(&mut wave, &tail);

    assert_eq!(wave, vec![0.0, 0.1, 0.0, -1.0]);
    // extend(&mut wave, &wave);   // waveはすでに可変参照として借用されている（第1引数）ので、共有参照として借用（第2引数）できない


    let mut w = (136, 139);
    let r = &mut w;
    let r0 = &mut r.0; // 可変参照から可変参照の再借用は出来る
    let r1 = &mut r.1; // 可変参照から可変参照の再借用は出来る
    *r0 = 10;

    // w.0; // 可変参照の借用中はアクセスできない
}

fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
    for elt in slice {
        vec.push(*elt);
    }
}
