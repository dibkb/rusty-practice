fn main() {
    let sentence = "Hello";
    let mut ans = String::new();
    for (_,c) in sentence.chars().enumerate(){
        if c != ' ' {
            ans.push(c);
        }else{
            break;
        }
    }
    println!("{}",ans);
}
