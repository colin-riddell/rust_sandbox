pub fn run(){
    let mut name = String::from("colin");
    greeting(&mut name, "nice to meet you")
}

pub fn greeting(greet: &mut String, name: &str){
    println!("Hello {}  {}", greet, name);
    greet.push_str("he");

}