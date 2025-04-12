struct User {
    first_name: String,
    last_name: String,
    age: i32,
}

// A more advanced struct example with methods and a constructor
struct Rectangle {
    width: f64,
    height: f64,
}

// Implementation block for Rectangle
impl Rectangle {
    // Constructor (associated function)
    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }
    
    // Method that calculates area
    fn area(&self) -> f64 {
        self.width * self.height
    }
    
    // Method that checks if this rectangle can contain another
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    // Method that creates a square (another associated function)
    fn square(size: f64) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main(){
    let user = User{
        first_name:String::from("Rohan"),
        last_name:String::from("Bhosale"),
        age:26,
    };
    println!("UserInfo = {} {} {}",user.first_name, user.last_name, user.age);
    println!("Hello, world!");
    println!("{}",user.first_name);         
    println!("{}",user.last_name);
    println!("{}",user.age);
}

fn rectangle_example() {
    // Create rectangles using different approaches
    let rect1 = Rectangle { width: 30.0, height: 50.0 };
    let rect2 = Rectangle::new(10.0, 20.0);
    let square = Rectangle::square(25.0);
    
    println!("Rectangle 1 area: {}", rect1.area());
    println!("Rectangle 2 area: {}", rect2.area());
    println!("Square area: {}", square.area());
    
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect1? {}", rect2.can_hold(&rect1));
}
