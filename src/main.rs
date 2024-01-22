fn main() {
    println!("Hello world!");

    let result = calculator::calculate_once(3_f64, "cos(2*x)-1/3");
    println!("{:?}", result);

    calculator::calculate(calculator::float_range(0.0, 10.0, 0.1).collect(), "cos(2*x)-1/3").iter()
        .for_each(|res| println!("{:?}", res));
}