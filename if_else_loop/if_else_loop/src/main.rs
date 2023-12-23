//change constant value to meet requirements.
const x:i32 = 10;

fn main() {
    less_than_greater_than_equal_to();
}

fn less_than_greater_than_equal_to(){
    if x < 10 {
    println!("{} is less than 10", x);}
    
    else if x == 10 { println!("{} is equal to 10", x); }
    
    else{ println!("{} is greater than 10", x);} 
}


