#[derive(Clone)]
struct Person {
    name: String,
    age: u16,

}
impl Person {
    fn new(name: &str, age: u16) -> Person {
        Person {
            name: name.to_string(),
            age: age,
        }
    }
    fn greet(&self) {
        println!("Hi, {}", self.name);
    }
}


pub fn run() {
    let mut person = Person::new("Hitler", 1488);
    let mut person2 = person.clone();
    person2.name = "Stalin".to_string();
    person2.greet();
    person.greet();
}