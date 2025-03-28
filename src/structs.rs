struct User {
    first_name: String,
    last_name: String,
    age: i32,
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

