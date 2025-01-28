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

fn dbg_macro(){
    let arr = [1,2,3,4];
    println!("printing debug print");
    let passed = dbg!(arr); // print it and pass it through
    println!("{:?}", passed);
}

fn array_test(){
    let mut arr: [i8; 4] = [1,2,3,4];
    arr[3] = 14;
    println!("{:?}", arr);
}

fn tuple_test(){
    let t: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = t;
    println!("{x}, {y}, {z}");
}

// fn borrowing_reference_test(){
//     let mut x = 'a';
//     let y = &x; // ref
//     println!("borrowing_reference_test: {y}");
//     *y = 'd';
// }

fn mutable_reference_test(){
    let mut x = 'a';
    let y = &mut x; // ref
    println!("mutable_reference_test: {y}");
    *y = 'd';
    println!("mutable_reference_test: {y}");
}

fn slice_test(){
    let arr = [1,2,3,4];
    println!("{:?}", arr);
    let slice = &arr[1..3];
    println!("{:?}", slice);
}

fn string_test(){
    let s: &str = "World";
    println!("hello {}", s);

    // owned
    let mut s2: String = String::from("World");
    s2.push_str("!");
    println!("hello {}", s2);
}

// fn reference_validity_test(){
//     let x_ref = {
//         let x = 5;
//         &x // pass the reference to x_ref, panics rust!
//     }
// }

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
    dbg_macro();
    array_test();
    tuple_test();
    // borrowing_reference_test(); //panic: `y` is a `&` reference, so the data it refers to cannot be written
    mutable_reference_test();
    slice_test();
    string_test();
    // reference_validity_test(); // `x` does not live long enough
}