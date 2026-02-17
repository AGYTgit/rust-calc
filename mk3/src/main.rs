use std::fmt;

#[derive(Clone)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl fmt::Debug for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Sub => write!(f, "-"),
            Operator::Mul => write!(f, "*"),
            Operator::Div => write!(f, "/"),
        }
    }
}

impl Operator {
    fn from_char(oper: char) -> Option<Operator> {
        match oper {
            '+' => Some(Operator::Add),
            '-' => Some(Operator::Sub),
            '*' => Some(Operator::Mul),
            '/' => Some(Operator::Div),
            _ => None,
        }
    }

    fn evaluate(&self, num1: i32, num2: i32) -> i32 {
        match self {
            Operator::Add => num1 + num2,
            Operator::Sub => num1 - num2,
            Operator::Mul => num1 * num2,
            Operator::Div => match num2 {
                0 => panic!("Division by 0!"),
                _ => num1 / num2,
            }
        }
    }
}

#[derive(Clone)]
enum Expression {
    Number(i32),
    Operation {
        operator: Operator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
}

impl fmt::Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Number(num) => write!(f, "{}", num),
            Expression::Operation {
                operator,
                left,
                right,
            } => write!(f, "({:?} {:?} {:?})", left, operator, right),
        }
    }
}

impl Expression {
    fn resolve(&self) -> i32 {
        match self {
            Expression::Number(num) => *num,
            Expression::Operation {
                operator,
                left,
                right,
            } => {
                let left_val = left.resolve();
                let right_val = right.resolve();
                operator.evaluate(left_val, right_val)
            }
        }
    }
}

fn main() {
    let mut input: String;
    loop {
        input = read_line();
        match validate_paren(&input) {
            true => break,
            false => println!("Invalid paren order!")
        };
    }
    let valid_input: String = add_paren_to_expression(&input);
    // let valid_input: String = input;

    let expr = construct_expression(&valid_input, &get_paren_tuple_indices(&valid_input));
    let result = expr.resolve();
    println!("{:?} = {}", expr, result);

    // let e = Expression::Operation {
    //     operator: Operator::Mul,
    //     left: Box::new(Expression::Number(3)),
    //     right: Box::new(Expression::Operation {
    //         operator: Operator::Add,
    //         left: Box::new(Expression::Number(1)),
    //         right: Box::new(Expression::Number(2)),
    //     }),
    // };
    //
    // let e_str = format!("{:?}", e);
    // println!("{}", e_str);
    // if !validate_paren(&e_str) { panic!("invalid paren") }
    // let paren_tuples: Vec<(i32, i32)> = get_paren_tuple_indices(&e_str);

    // let tokens: Vec<&str> = input.split_whitespace().collect();
}

fn read_line() -> String {
    use std::io::stdin;

    let mut s = String::new();
    stdin().read_line(&mut s).expect("Failed to read from stdin!");
    s.trim().to_string()
}

fn validate_paren(input: &String) -> bool {
    let mut left: usize = 0;
    let mut right: usize = 0;

    for c in input.chars() {
        match c {
            '(' => left += 1,
            ')' => right += 1,
            _ => continue,
        };

        if left < right { return false; }
    }

    if left == right { return true; }

    false
}

fn get_paren_tuple_indices(input: &String) -> Vec<(usize, usize)> {
    let mut left: Vec<usize> = Vec::new();
    let mut right: Vec<usize> = Vec::new();

    for i in 0..input.len() {
        match input.chars().nth(i).unwrap() {
            '(' => left.push(i.try_into().unwrap()),
            ')' => right.push(i.try_into().unwrap()),
            _ => (),
        };
    }
    let left = left;
    let right = right;

    let mut tuples: Vec<(usize, usize)> = Vec::new();
    let mut right_used: Vec<usize> = Vec::new();
    for i in (0..left.len()).rev() {
        let l = left.get(i).unwrap();
        for j in 0..right.len() {
            let r = right.get(j).unwrap();
            if r < l { continue }
            match right_used.contains(r) {
                true => continue,
                false => {
                    right_used.push(*r);
                    tuples.push((*l, *r));
                    break;
                }
            };
        }
    }

    tuples.iter().copied().rev().collect()
}

fn add_paren_to_expression(input: &String) -> String { // BUG: A LOT OF THEM
    let input: String = input.clone();
    let number_token_count: usize = input.split_whitespace().collect::<Vec<_>>().len() / 2;

    let paren_count: usize = {
        let mut left_paren_count: usize = 0;
        for c in input.chars() {
            if c == '(' {
                left_paren_count += 1;
            }
        }
        left_paren_count
    };

    if number_token_count < paren_count {
       panic!("invalid paren");
    } else if number_token_count == paren_count {
        return input;
    }

    let mut input = input;
    let missing_paren = number_token_count - paren_count;

    input = format!("{}{}", "(".repeat(missing_paren), input);
    println!("{}", input);

    let mut whitespace_count: i32 = 0;
    for i in 0..input.len() {
        println!("{}: {}", i, input.chars().nth(i).unwrap());
        if input.chars().nth(i).unwrap() == ' ' {
            whitespace_count += 1;
        }

        if whitespace_count == 3 {
            whitespace_count = 0;
            if input.chars().nth(i - 1).unwrap() == ')' {
                continue;
            }
            if input.chars().nth(i - 2).unwrap() == '(' {
                continue;
            }
            if input.chars().nth(i - 4).unwrap() == '(' {
                continue;
            }
            input.insert(i, ')');
        }
    }
    input = format!("{})", input);

    println!("{}", input);
    input
}

fn construct_expression(input: &String, paren_tuples: &Vec<(usize, usize)>) -> Expression {
    let paren_tuples: Vec<(usize, usize)> = paren_tuples.to_vec();
    println!("{:?}", paren_tuples);

    let mut expr_left: Option<Expression> = None;
    let mut expr_right: Option<Expression> = None;
    let mut i = 0;
    for tuple_index in (0..paren_tuples.len()).rev() {
        match i {
            0 => {
                let tuple = paren_tuples.get(tuple_index).unwrap();
                expr_right = Some(construct_expression_segment_from_slice(&input[tuple.0 + 1..tuple.1]));
            },
            _ => {
                let tuple = paren_tuples.get(tuple_index).unwrap();
                let tuple_prev = paren_tuples.get(tuple_index + 1).unwrap();
                // check for overlap with 1
                match expr_left.as_ref() {
                    None => {
                        if tuple.0 < tuple_prev.0 && tuple.1 > tuple_prev.1 {
                            let left_part = &input[tuple.0 + 1..tuple_prev.0];
                            // let right_part = &input[tuple_prev.1 + 1..tuple.1]; // BUG: UNUSED???
                            if left_part.trim().is_empty() {
                                // left inside
                                expr_right = Some(construct_expression_segment_from_expr_and_slice(&input[tuple_prev.1 + 1..tuple.1], &expr_right.as_ref().unwrap()));
                            } else {
                                // right inside
                                expr_right = Some(construct_expression_segment_from_slice_and_expr(&input[tuple.0 + 1..tuple_prev.0 - 1], &expr_right.as_ref().unwrap()));
                            }
                        } else {
                            // outside
                            expr_left = Some(construct_expression_segment_from_slice(&input[tuple.0 + 1..tuple.1]));
                        }
                    },
                    Some(_) => {
                        let tuple_prev_prev = paren_tuples.get(tuple_index + 2).unwrap();
                        expr_right = Some(construct_expression_segment_from_expr_and_expr(&input[tuple_prev.1 + 1..tuple_prev_prev.0 - 1], &expr_left.as_ref().unwrap(), &expr_right.as_ref().unwrap()));
                    },
                };
            },
        }
        i += 1;
    }

    expr_right.unwrap()
}

fn construct_expression_segment_from_slice(input: &str) -> Expression {
    let tokens: Vec<&str> = input.split_whitespace().collect();
    println!("{:?}", tokens);
    let num1: i32 = tokens[0].parse().unwrap();
    let oper: Operator = Operator::from_char(tokens[1].chars().nth(0).unwrap()).unwrap();
    let num2: i32 = tokens[2].parse().unwrap();
    Expression::Operation {
        operator: oper,
        left: Box::new(Expression::Number(num1)),
        right: Box::new(Expression::Number(num2)),
    }
}

fn construct_expression_segment_from_slice_and_expr(input: &str, expr: &Expression) -> Expression {
    let tokens: Vec<&str> = input.split_whitespace().collect();
    println!("{:?}", tokens);
    let num1: i32 = tokens[0].parse().unwrap();
    let oper: Operator = Operator::from_char(tokens[1].chars().nth(0).unwrap()).unwrap();
    Expression::Operation {
        operator: oper,
        left: Box::new(Expression::Number(num1)),
        right: Box::new(expr.clone()),
    }
}

fn construct_expression_segment_from_expr_and_slice(input: &str, expr: &Expression) -> Expression {
    let tokens: Vec<&str> = input.split_whitespace().collect();
    println!("{:?}", tokens);
    let oper: Operator = Operator::from_char(tokens[0].chars().nth(0).unwrap()).unwrap();
    let num2: i32 = tokens[1].parse().unwrap();
    Expression::Operation {
        operator: oper,
        left: Box::new(expr.clone()),
        right: Box::new(Expression::Number(num2)),
    }
}

fn construct_expression_segment_from_expr_and_expr(input: &str, expr_left: &Expression, expr_right: &Expression) -> Expression {
    let tokens: Vec<&str> = input.split_whitespace().collect();
    println!("{:?}", tokens);
    let oper: Operator = Operator::from_char(tokens[0].chars().nth(0).unwrap()).unwrap();
    Expression::Operation {
        operator: oper,
        left: Box::new(expr_left.clone()),
        right: Box::new(expr_right.clone()),
    }
}
