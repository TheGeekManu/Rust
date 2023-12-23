fn main() {
    bitwise_rightshift();
}

fn bitwise_rightshift() {
    let mut x: i32 = 128;
    x = x >> 2;
    println!("{}", x);
}
