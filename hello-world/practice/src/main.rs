enum Shape{
    Square(f64),
    Circle(f64),
    Rect(f64,f64)
}

fn calculate_area(shape: &Shape)-> f64{
    let pi = std::f64::consts::PI;
    match shape {
        Shape::Circle(radius)=> radius * radius *  pi,
        Shape::Square(a)=> a * a,
        Shape::Rect(a,b)=> a*b
    }
}
fn main() {
    let square = Shape::Square(32.21);
    println!("Area of the square with side {} is {}", 32.21, calculate_area(&square));
    
}


