fn main() {
    let a: i64 = 10;  
    let b: i64 = 20;

    let c: i64 = 50;
    println!("these type we are using these interger {}",c);
    // "{}",c is a placeholder for the value of c



    println!("addition {}", a + b);
    println!("subtraction {}", a - b);
    println!("multiplication {}", a * b);
    println!("division {}", a / b);
    println!("modulus {}", a % b);

    println!("these a A number {}", a);
    println!("these b A number {}", b);

    println!("this is a string");
    println!("testing the print function for the usage");

    println!("let start from the numbers, then boolen then string then characters then tuples.");

    println!("let's start with the numbers");

    let a:i64 = 10;
    let b:i64 = 20;
    println!("addition {}", a + b);
    println!("subtraction {}", a - b);
    println!("multiplication {}", a * b);
    println!("division {}", a / b);
    println!("modulus {}", a % b);

    println!("Now let's try some type conversion");
    
    let emp_info:(&str,u8) = ("Ramesh", 30);

    let emp_name = emp_info.0;
    let emp_age = emp_info.1;

    //destructuring

    let (employee_name, employee_age) = emp_info;
    println!("{} is {} years old", employee_name, employee_age);


    println!("{} is {} years old", emp_name, emp_age);
    print_value();

}


fn print_value(){

    let num1: u8 = 10;
    let num2: u8 = 20;  
    let result: u8 = addition(num1, num2);
    println!("the sum of two numbers is {}", result);
    
}

fn addition(item1:u8, item2:u8) -> u8 {
    return item1 + item2;
}
