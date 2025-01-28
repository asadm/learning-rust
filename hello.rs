fn overflow_test(){
    let mut x: u8 = 255;
    x+=1;
    println!("hello world {}", x);
}
fn mutation_test(){

    let mut x = 5_000;
    println!("hello world {}", x);
    x+=1;
    println!("hello world {}", x);

}
fn main(){
    mutation_test(); // fails if `mut` is not present
    overflow_test(); // panic: attempt to add with overflow
}