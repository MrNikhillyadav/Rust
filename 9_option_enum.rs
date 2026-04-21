
fn find_first_a(word:&String)->Option<usize> {
    for (index, character) in word.chars().enumerate(){
        if character == 'a' {
            return Some(index);
        }
    }
    return None;
}


fn main(){
    let my_string = String::from("Krishna");

    match find_first_a(&my_string) {
        Some(index) => {
            println!("{}", index);
        },
        None => println!("the letter do not contains a")
    }

    println!("{}", my_string);
    
}