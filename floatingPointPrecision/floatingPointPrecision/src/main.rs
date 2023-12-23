fn main() {
    floating_point_precision();
}

fn floating_point_precision(){
    //should panic when conducting below operation.
    //assert!(0.1 + 0.2 == 0.3);
    
    //step one
    assert!(0.1_f32 + 0.2_f32 == 0.3_f32);
    println!("First method success.");
    assert!(0.1 as f32 + 0.2 as f32 == 0.3 as f32);
    println!("Second method success.");

}
