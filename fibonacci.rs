fn fib(n:u32) -> u32{
    if n < 2{
        // base case
        return n;
    }
    else{
        // recursive case
        return fib(n-1) + fib(n-2);
    }
}

fn main(){
    let inp = 40;
    let n = fib(inp);
    println!("fib({}) = {}", inp, n);
}
