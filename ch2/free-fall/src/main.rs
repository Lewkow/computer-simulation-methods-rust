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
        y += numerical_update_y(v, dt);
        v += numerical_update_v(g, dt);
        t += dt;
    }

    println!("results:");
    println!("final time = {}", t);
    println!("numerical");
    println!("y = {}", y);
    println!("v = {}", v);
    let y_theory : f64 = theoretical_y(y0, v0, g, t);
    let v_theory : f64 = theoretical_v(v0, g, t);
    println!("theoretical");
    println!("y = {}", y_theory);
    println!("v = {}", v_theory);
}

fn theoretical_y(y0: f64, v0: f64, g: f64, t: f64) -> f64 {
    let ret = y0 + v0 * t - 0.5 * g * t.powi(2);
    return ret;
}

fn theoretical_v(v0: f64, g: f64, t: f64) -> f64 {
    let ret = v0 - g * t;
    return ret;
}

fn numerical_update_v(g: f64, dt: f64) -> f64 {
    let ret = -g * dt;
    return ret;
}

fn numerical_update_y(v: f64, dt: f64) -> f64 {
    let ret = v * dt;
    return ret;
}