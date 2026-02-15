fn main() {
    let mut result: i32 = match read_int() {
        Some(i) => i,
        None => return
    };

    let move_cursor_up: &str = "\x1b[1A";
    loop {
        let input_char: char = match read_char() {
            Some(c) => c,
            None => {print!("{}", move_cursor_up); break}
        };

        let oper_fn: fn(i32, i32) -> i32 = match get_oper_fn(input_char) {
            Some(f) => f,
            None => {println!("not a valid operator!"); return}
        };

        let input_int: i32 = match read_int() {
            Some(i) => i,
            None => {print!("{}", move_cursor_up); break}
        };

        result = oper_fn(result, input_int);
    }

    println!("result: {}", result);
}

fn get_oper_fn(c: char) -> Option<fn(i32, i32) -> i32> {
    match c {
        '+' => Some(add),
        '-' => Some(sub),
        '*' => Some(mul),
        '/' => Some(div),
        _ => None
    }
}

fn add(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

fn sub(num1: i32, num2: i32) -> i32 {
    num1 - num2
}

fn mul(num1: i32, num2: i32) -> i32 {
    num1 * num2
}

fn div(num1: i32, num2: i32) -> i32 {
    match num2 {
        0 => panic!("Division by 0!"),
        _ => num1 / num2
    }
}

fn read_int() -> Option<i32> {
    use std::io::stdin;

    let mut s: String = String::new();
    stdin().read_line(&mut s).expect("Failed to read!");

    let trimmed: &str = s.trim();

    if trimmed.is_empty() {
        return None;
    }

    match trimmed.to_string().parse::<i32>() {
        Ok(i) => Some(i),
        Err(_) => panic!("Failed to parse i32!")
    }
}

fn read_char() -> Option<char> {
    use std::io::stdin;

    let mut s: String = String::new();
    stdin().read_line(&mut s).expect("Failed to read!");

    let trimmed: &str = s.trim();

    if trimmed.is_empty() {
        return None;
    }

    match trimmed.to_string().parse::<char>() {
        Ok(c) => Some(c),
        Err(_) => panic!("Failed to parse char!")
    }
}
