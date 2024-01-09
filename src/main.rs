use calculator::calculate;

fn main() {
    println!("Hello world!");

    let result = calculate(3_f64, "cos(2*x)-1/3");
    println!("{:?}", result);
}