// enum Shape {
//     Rectangle(f64, f64),
//     Circle(f64),
// }

// fn main(){
//     let rect = Shape::Rectangle(10.0, 20.0);
//     let circle = Shape::Circle(10.0);
//     println!("Area = {}", calculate_area(rect));
//     println!("Area = {}", calculate_area(circle));

// }

// fn calculate_area(shape: Shape) -> f64 {
//     match shape {
//         Shape::Rectangle(a, b) => a * b,
//         Shape::Circle(r) => 3.14 * r * r,
//     }
// }


enum Direction{
    North,
    South,
    East,
    West
}
fn move_player(direction: Direction){
    match direction {
        Direction::North => println!("Going North"),
        Direction::South => println!("Going South"),
        Direction::East => println!("Going East"),
        Direction::West => println!("Going West"),
    }
}

fn main(){
    let dir = Direction::North;  
    move_player(dir);   
}