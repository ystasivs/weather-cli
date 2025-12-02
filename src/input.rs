use std::io::{self, Write};

pub fn read_user_number(min: usize, max: usize, max_attempts: u32) -> Result<usize, String> {
    let mut attempts = 0;

    while attempts < max_attempts {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<usize>() {
            Ok(n) if n >= min && n <= max => return Ok(n),
            _ => {
                attempts += 1;
                println!(
                    "Invalid input ({} of {} attempts). Please enter a number between {} and {}.",
                    attempts, max_attempts, min, max
                );
            }
        }
    }

    Err(format!("Failed to provide a valid number after {} attempts", max_attempts))
}

pub fn read_user_string() -> String {
    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}