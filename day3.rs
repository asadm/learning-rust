fn say_hello(name: String) {
    println!("Hello {name}")
}

fn transfer_ownership_test() {
    let name = String::from("Alice");
    say_hello(name);
    // say_hello(name); // panic: value used here after move
}

fn clone_test(){
    let name = String::from("Clone");
    say_hello(name.clone());
    say_hello(name);
}

#[derive(Copy, Clone, Debug)]
struct Point(i32, i32);

fn copy_test(){
    let mut p1 = Point(1,2);
    let p2 = p1; // copy
    println!("p1: {p1:?}");
    println!("p2: {p2:?}");

    p1.0 = 3;
    p1.1 = 4;
    println!("p1: {p1:?}");
    println!("p2: {p2:?}");
}

#[derive(Debug)]
struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Dropping {}", self.name);
    }
}

fn drop_test(){
    let a = Droppable { name: "a" };
    {
        let b = Droppable { name: "b" };
        {
            let c = Droppable { name: "c" };
            println!("Exiting scope:{c:?}");
        }
        println!("Exiting scope:{b:?}");
    }
    // std::mem::drop(a); // can drop manually
    println!("Exiting scope:{a:?}");
}

fn box_test(){
    let b = Box::new(10);
    println!("box: {b}");
    println!("box {}", *b);
}

use std::rc::Rc;
fn rc_test(){
    let a = Rc::new(String::from("hello"));
    let b = Rc::clone(&a);
    println!("a: {a}");
    println!("b: {b}");
}

/////
struct Dog {
    name: String,
    age: i8
}

struct Cat {
    lives: i8
}

trait Pet {
    fn talk(&self) -> String;
}

impl Pet for Cat {
    fn talk(&self) -> String {
        format!("Meow, I have {} lives!", self.lives)
    }
}

impl Pet for Dog {
    fn talk(&self) -> String {
        format!("Woof, my name is {}!", self.name)
    }
}

fn owned_trait_test(){ // dyn vec 
    let pets: Vec<Box<dyn Pet>> = vec![
        Box::new(Dog{name: String::from("Pup!"), age: 1}),
        Box::new(Cat{lives: 9}),
    ];
    for pet in pets{
        println!("{}", pet.talk());
    }
}

fn borrow_checker_test(){
    let x_ref = {
        let x = 10;
        &x // panic: borrowed value does not live long enough
    };

    println!{"x_ref: {x_ref}"};
}

fn main(){
    transfer_ownership_test();
    clone_test();
    copy_test();
    drop_test();
    box_test();
    rc_test();
    owned_trait_test();
    borrow_checker_test();
}