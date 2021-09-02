
fn fizzbuzz(number:i32)-> String {
    if number % 3 == 0 && number % 5 == 0{
        String::from("fizzbuzz")
    }
    else if number % 3 == 0{
        String::from("fizz")
    } 
    else if number % 5 == 0{
        String::from("buzz")
    } else {
        String::from(number.to_string())
    }
}


pub fn run(){
    for number in 0..100 {
        let mut answer = fizzbuzz(number);
        answer = format!("answer is {}", number.to_string());
        println!("{}", answer)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_fizz_on_3(){
        let out = fizzbuzz(3);
        assert_eq!(out, "fizz")
    }

    #[test]
    fn it_returns_buzz_on_5(){
        let out = fizzbuzz(5);
        assert_eq!(out, "buzz")
    }


    #[test]
    fn it_returns_1_on_1(){
        let out = fizzbuzz(1);
        assert_eq!(out, "1")
    }

    #[test]
    fn it_returns_fizzbuzz_on_15(){
        let out = fizzbuzz(15);
        assert_eq!(out, "fizzbuzz")
    }
}