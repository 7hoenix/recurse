fn main() {
    let mut v = vec![1];
    v.push(2);
    for i in &mut v {
        *i += 40;
    }
    v[88];
}
