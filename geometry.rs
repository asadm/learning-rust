// Calc magnitude of vector by summing the squares of its coordinated and taking sqrt.
fn magnitude(vector: &[f64]) -> f64{
    let sum = vector.iter().map(|x| x*x).sum::<f64>();
    sum.sqrt()
}

fn normalize(vector: &mut [f64]){
    let mag = magnitude(vector);
    for x in vector.iter_mut(){
        *x /= mag;
    }
}


fn main(){
    println!("Magnitude of a unit vector: {}", magnitude(&[0.0, 1.0, 0.0]));

    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));
}