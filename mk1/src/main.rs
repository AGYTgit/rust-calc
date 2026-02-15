fn main() {
    let mut num: i32 = 0;

    loop {
        let input: Option<i32> = read_line();
        match input {
            Some(i) => num += i,
            None => break
        }
    }

    let move_cursor_up: &str = "\x1b[1A";
    print!("{}sum: {}\n", move_cursor_up, num);
}

fn read_line() -> Option<i32> {
    use std::io::stdin;

    let mut s: String = String::new();
    stdin().read_line(&mut s).expect("Failed to read!");

    let trimmed: &str = s.trim();

    if trimmed.is_empty() {
        return None;
    }

    match trimmed.to_string().parse::<i32>() {
        Ok(num) => Some(num),
        Err(_) => panic!("Failed to parse i32!")
    }
}
