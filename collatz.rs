fn collatz_length(mut n: u32) -> u32{
    assert!(n>0, "n must be positive");
    let mut count = 1;
    while n!=1{
        dbg!(n);
        if n%2==0{
            n/=2;
        }
        else{
            n = 3*n+1;
        }
        count+=1;
    }
    count
}

fn main(){
    let n = collatz_length(11);
    println!("collatz_length is {n}");
}