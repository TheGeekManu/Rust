fn main() {
    //initialize x
    let x: i32 = 5;
    //initialize y
    let mut y: i32 = 9;
    //increment
    y += 1;
    //compare
    assert_eq!(x, 5);
    assert_eq!(y, 10);
    println!("Comparison is true!");
    println!("x is : {}, y is : {}", x, y);
}
