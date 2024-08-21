fn practice_bool(){
    let age:u32 = 23;
    let is_adult = age >= 18;
    if is_adult {
        println!("You are an adult.");
    } else {
        println!("You are not an adult.");
    }
}
fn practice_string(){
    let str = String::from("Hello");
    let str: String = extract_first_word(str);
    println!("{}",str)
}
fn extract_first_word(sentence:String)->String{
    let mut ans:String = String::new();
    for char in sentence.chars(){
        if char == ' ' {
            break;
        }
        ans.push_str(char.to_string().as_str())
    }
    return ans;
}
fn main() {
    practice_string();
}

