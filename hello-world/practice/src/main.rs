struct User {
    first_name : String,
    last_name : String,
    email : String,
}
fn print_user_info(user:&User){
    println!("First Name : {}",user.first_name);
    println!("Last Name : {}",user.last_name);
    println!("Email : {}",user.email);
}

fn main() {
    let user_one: User = User{
        first_name : String::from("Ramesh"),
        last_name : String::from("Gol"),
        email : String::from("ramesh@mail.com"),
    };

    print_user_info(&user_one);
}


