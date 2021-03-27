fn main() {
    println!("simulating free fall");
    let y0 : f64 = 10.0;
    let v0 : f64 = 0.0;
    let dt : f64 = 0.01;
    let g : f64 = 9.8;
    let mut t : f64 = 0.0;
    let mut y : f64 = y0;
    let mut v : f64 = v0;

    for _n in 0..100 {
        y += v * dt;
        v -= g * dt;
        t += dt;
    }

    println!("results:");
    println!("final time = {}", t);
    println!("numerical");
    println!("y = {}", y);
    println!("v = {}", v);
    let y_theory : f64 = y0 + v0 * t - 0.5 * g * t.powi(2);
    let v_theory : f64 = v0 - g * t;
    println!("theoretical");
    println!("y = {}", y_theory);
    println!("v = {}", v_theory);
}