//use std::cmp::Ordering; // rust warns of unused if this is uncommented


fn min<T: Ord>(x: T, y: T) -> T {
    x.min(y)
}

fn main() {
    assert_eq!(min(0, 10), 0);
    assert_eq!(min(500, 123), 123);

    assert_eq!(min('a', 'z'), 'a');
    assert_eq!(min('7', '1'), '1');

    assert_eq!(min("hello", "goodbye"), "goodbye");
    assert_eq!(min("bat", "armadillo"), "armadillo");
    println!("Success!");
}