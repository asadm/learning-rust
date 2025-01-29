use std::collections::HashMap;

#[derive(Debug)]
struct Car {
    name: String,
    color: String,
    health: i32,
}

impl Car{
    // no constructors in rust, rust has factory methods
    fn new(name: String, color: String, health: i32) -> Self{
        Self{name, color, health}
    }

    fn change_health(&mut self, health: i32){
        self.health = health;
    }
}

fn methods_test(){
    let mut car = Car::new(String::from("Tesla"), String::from("Red"), 100);
    println!("{:?}", car);
    car.change_health(200);
    println!("{:?}", car);
}

trait Human {
    fn talk(&self) -> String;
    fn walk(&self, x: i32, y: i32) -> String;
}

struct Asad{
    name: String,
    age: i32,
}

impl Human for Asad{
    fn talk(&self) -> String {
        "Hello".to_string()
    }

    fn walk(&self, x: i32, y: i32) -> String {
        format!("Walking to ({x}, {y})")
    }
}

fn trait_test(){
    let asad = Asad{
        name: String::from("Asad"),
        age: 27,
    };
    println!("{}", asad.talk());
    println!("{}", asad.walk(1,2));
}

fn generic_test<T>(n: i32, x: T, y: T) -> T{
    if n%2==0 {
        x
    }
    else{
        y
    }
}

fn trait_bound_test<T: std::fmt::Debug>(x: T){
    println!("I am sure this is dbg printable: {:?}", x);
}

fn vec_test(){
    let mut v = vec![1,2,3,4];
    println!("{:?}", v);
    println!("len: {}, cap: {}", v.len(), v.capacity());
    v.push(5);
    println!("len: {}, cap: {}", v.len(), v.capacity());
}

fn hashmap_test(){
    let mut map = HashMap::new();
    map.insert(1, "one");
    map.insert(2, "two");
    println!("{:?}", map);
    let a = map.get(&1);
    println!("{:?}", a);
}


fn main(){
    methods_test();
    trait_test();
    println!("{}", generic_test(1, "even", "odd"));
    trait_bound_test(1);
    trait_bound_test("hello");
    vec_test();
    hashmap_test();
}