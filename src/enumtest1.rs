enum Shape {
    Rectangle(f64, f64),
    Circle(f64),
}

fn main(){
    let rect = Shape::Rectangle(10.0, 20.0);
    let circle = Shape::Circle(10.0);
    println!("Area = {}", calculate_area(rect));
    println!("Area = {}", calculate_area(circle));

}

fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Rectangle(a, b) => a * b,
        Shape::Circle(r) => 3.14 * r * r,
    }
}