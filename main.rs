use std::io;

fn input() -> String {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("error reading input");
    inp.trim().to_string()
}

struct CLI {
    command: String,
    flag1: 
}


fn main() {
    let command = input();
    if command.contains(&"cab ".to_string()) {
        println!("{}", &command[4..]);
    }
}