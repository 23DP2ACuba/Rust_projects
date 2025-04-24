use std::cmp::Ordering;
use std::io;
use std::io::Write;
use std::error::Error;
use rand::Rng;

macro_rules! print_flush {
    ($($arg:tt)*) => {
        print!($($arg)*);
        if true {
            io::stdout().flush().expect("Failed to flush stdout");
        }
    };
}

fn read_inp() -> Result<u16, Box<dyn Error>>{
    let mut inp: String = String::new();
    io::stdin().read_line(&mut inp)?;
    let inp: u16 = inp.trim().parse()?;
    Ok(inp)
}
fn compare(actual: &u16, guess: &u16) -> Result<String, Box<dyn Error>> {
    match guess.cmp(&actual) {
        Ordering::Less => return Ok("Too small!".to_string()),
        Ordering::Greater => return Ok("Too big!".to_string()),
        Ordering::Equal => return Ok("You win!".to_string()),
    }
}

fn auto_guess(min: u16, max: u16, res: &str) -> Result<u16, Box<dyn Error>> {
    let mut curr = (min + max) / 2;
    match res {
        "Too small!" => curr = (curr + max) / 2,
        "Too big!" => curr = (min + curr) / 2,
        _ => {},
    };
    Ok(curr)
}

fn game() -> Result<(), Box<dyn Error>>{
    print_flush!("Enter game mode (auto: 0, player: 1): ");
    let mode = read_inp()?;
    print_flush!("Enter min: ");
    let mut min: u16 = read_inp()?;
    print_flush!("Enter max: ");
    let mut max: u16 = read_inp()?;
    let mut guess: u16 = 0;

    let actual = rand::thread_rng().gen_range(min..=max);
    let mut res: String = String::new();
    println!("Guess a number");
    loop{
        match mode {
            0 => guess = auto_guess(min, max, &res)?,
            1 => {
                print_flush!("Please input your guess: ");
                guess = read_inp()?;
            },
            _ => guess = auto_guess(min, max, &res)?,
        };
        println!("guessed: {}", guess);
        res = compare(&actual, &guess)?;
        println!("{}", res);
        if res == "You win!".to_string() {
            return Ok(());
        } else if res == "Too small!".to_string() && mode == 0{
            min = guess + 1;
        } else if res == "Too big!".to_string() && mode == 0{
            max = guess - 1;
        }
        
    };

}

pub fn run_game() {
    _ = game();
}
