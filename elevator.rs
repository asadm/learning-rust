// A simple event enum for an Elevator

#[derive(Debug)]
/// An event in the elevator system that the controller must react to.
enum Event {
    CarArrived(i32),
    CarDoorOpen(bool),
    ButtonPressed(Button)
}

/// A direction of travel.
#[derive(Debug)]
enum Direction {
    Up,
    Down,
}

#[derive(Debug)]
enum Button {
    LobbyFloor(Direction, i32),
    CarFloor(i32),
}

/// The car has arrived on the given floor.
fn car_arrived(floor: i32) -> Event {
    return Event::CarArrived(floor);
}

/// The car doors have opened.
fn car_door_opened() -> Event {
    return Event::CarDoorOpen(true);
}

/// The car doors have closed.
fn car_door_closed() -> Event {
    return Event::CarDoorOpen(false);
}

/// A directional button was pressed in an elevator lobby on the given floor.
fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event {
    return Event::ButtonPressed(Button::LobbyFloor(dir, floor));
}

/// A floor button was pressed in the elevator car.
fn car_floor_button_pressed(floor: i32) -> Event {
    return Event::ButtonPressed(Button::CarFloor(floor));
}

fn main() {
    println!(
        "A ground floor passenger has pressed the up button: {:?}",
        lobby_call_button_pressed(0, Direction::Up)
    );
    println!("The car has arrived on the ground floor: {:?}", car_arrived(0));
    println!("The car door opened: {:?}", car_door_opened());
    println!(
        "A passenger has pressed the 3rd floor button: {:?}",
        car_floor_button_pressed(3)
    );
    println!("The car door closed: {:?}", car_door_closed());
    println!("The car has arrived on the 3rd floor: {:?}", car_arrived(3));
}