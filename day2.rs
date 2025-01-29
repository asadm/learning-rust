#[derive(Debug)]
struct Car {
    name: String,
    color: String,
    health: i32,
}

impl Car{
    // no constructors in rust, rust has factory methods
    fn new(name: String, color: String, health: i32) -> Car{
        Car{name, color, health}
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

fn main(){
    methods_test();
}