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

fn no_ternary(){
    let x = 20;
    let size = if x> 20 { "so big"} else {"smol"};
    println!("hello world {}", size);
}

fn match_test(val:i32){
    match val {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4|5|6 => println!("four to six"), //multiples
        7..=10 => println!("seven to ten"), //range (inclusive 10 (with = sign))
        _ => println!("something else"),
    }
}

fn loop_test(){
    for x in [1,2,3,4]{
        println!("loop test {}", x);
    }
}

fn inf_loop(){
    let mut x = 5;
    loop{
        println!("hello world");
        x-=1;
        if x==0{
            break;
        }
    }
}

fn loop_as_expression(){
    let mut x = 5;
    let breakswith = loop {
        println!("loop_as_expression::inside loop");
        x-=1;
        if x==0{
            break x;
        }
    };
    println!("breakswith {breakswith}"); // println can have a var directly inside it
}

fn loop_label_break(){
    let mut x = 5;
    'outer: loop {
        println!("outer");
        loop {
            println!("inner");
            x-=1;
            if x==0{
                break 'outer;
            }
        }
    }
}

fn debug_print(){
    let arr = [1,2,3,4];
    println!("{:?}", arr);
}

fn main(){
    mutation_test(); // fails if `mut` is not present
    // overflow_test(); // panic: attempt to add with overflow
    no_ternary();
    match_test(7);
    loop_test();
    inf_loop();
    loop_as_expression();
    loop_label_break();
    debug_print();
}