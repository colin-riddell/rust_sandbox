struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Construct person
    fn new(first: &str, last: &str) -> Person {
       return Person {first_name: first.to_string(), last_name: last.to_string()}
    }

    // Get full name
    fn get_full_name(&self) -> String{
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set last name
    fn set_last_name(&mut self, name: &str){
        self.last_name = name.to_string();
    }
}

pub fn run() {

    let mut p = Person::new("Mary", "Doe");

    p.set_last_name("Williams");
    println!("Person {} ", p.get_full_name())

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_some_stuff() {
        let mut p = Person::new("Mary", "Doe");

        p.set_last_name("Williams");
        assert_eq!(p.get_full_name(), "Mary Williams");
    }
}