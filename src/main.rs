use std::io;

fn main() {

    let mut equate = String::new();

    loop {
    println!("Select an equation type: Add, Subtract, Multiply, Divide, (Case Sensitive: use words exactly as written)");
    equate.clear();

    io::stdin()
       .read_line(&mut equate)
       .expect("Input should match the given words.");
    match equate.trim() {
        "Add" => add(),
        "Subtract" => sub(),
        "Multiply" => mult(),
        "Divide" => div(),
        _ => panic!("Your input should match the given words."),
    }
  }
}

fn add() {
//    println!("addition successfull");
    let mut num1 = String::new();
    let mut num2 = String::new();

    loop {
    println!("Input your first digit (Cannot be decimal or negative)");
    
    io::stdin()
        .read_line(&mut num1)
        .expect("Invalid Number");

    let num1: u32 = match num1.trim().parse() {
       Ok(num) => num,
       Err(_) => break,
    };



    println!("Input your second digit (Cannot be decimal or negative)");
    
    io::stdin()
        .read_line(&mut num2)
        .expect("Invalid Number");

    let num2: u32 = match num2.trim().parse() {
        Ok(num) => num,
        Err(_) => break,
    };

    println!("{num1} + {num2} =");
    println!("{}", num1 + &num2);

    let mut equate = String::new();

    println!("Select an equation type: Add, Subtract, Multiply, Divide, (Case Sensitive: use words exactly as written)");
    equate.clear();

    io::stdin()
       .read_line(&mut equate)
       .expect("Input should match the given words.");
    match equate.trim() {
        "Add" => add(),
        "Subtract" => sub(),
        "Multiply" => mult(),
        "Divide" => div(),
        _ => panic!("Your input should match the given words."),
    }
  }
}
fn sub() {
//    println!("subtraction successfull");
let mut num1 = String::new();
let mut num2 = String::new();

loop {
println!("Input your first digit (Cannot be decimal or negative)");

io::stdin()
    .read_line(&mut num1)
    .expect("Invalid Number");

let num1: u32 = match num1.trim().parse() {
   Ok(num) => num,
   Err(_) => break,
};



println!("Input your second digit (Cannot be decimal or negative)");

io::stdin()
    .read_line(&mut num2)
    .expect("Invalid Number");

let num2: u32 = match num2.trim().parse() {
    Ok(num) => num,
    Err(_) => break,
};

println!("{num1} - {num2} =");
println!("{}", num1 - &num2);

let mut equate = String::new();

println!("Select an equation type: Add, Subtract, Multiply, Divide, (Case Sensitive: use words exactly as written)");
equate.clear();

io::stdin()
   .read_line(&mut equate)
   .expect("Input should match the given words.");
match equate.trim() {
    "Add" => add(),
    "Subtract" => sub(),
    "Multiply" => mult(),
    "Divide" => div(),
    _ => panic!("Your input should match the given words."),
    }
  }
}

fn mult() {
//    println!("multiplication successfull");
let mut num1 = String::new();
let mut num2 = String::new();

loop {
println!("Input your first digit (Cannot be decimal or negative)");

io::stdin()
    .read_line(&mut num1)
    .expect("Invalid Number");

let num1: u32 = match num1.trim().parse() {
   Ok(num) => num,
   Err(_) => break,
};



println!("Input your second digit (Cannot be decimal or negative)");

io::stdin()
    .read_line(&mut num2)
    .expect("Invalid Number");

let num2: u32 = match num2.trim().parse() {
    Ok(num) => num,
    Err(_) => break,
};

println!("{num1} * {num2} =");
println!("{}", num1 * &num2);

let mut equate = String::new();

println!("Select an equation type: Add, Subtract, Multiply, Divide, (Case Sensitive: use words exactly as written)");
equate.clear();

io::stdin()
   .read_line(&mut equate)
   .expect("Input should match the given words.");
match equate.trim() {
    "Add" => add(),
    "Subtract" => sub(),
    "Multiply" => mult(),
    "Divide" => div(),
    _ => panic!("Your input should match the given words."),
}
}
}

fn div() {
//    println!("division successfull");
let mut num1 = String::new();
let mut num2 = String::new();

loop {
println!("Input your first digit (Cannot be decimal or negative)");

io::stdin()
    .read_line(&mut num1)
    .expect("Invalid Number");

let num1: u32 = match num1.trim().parse() {
   Ok(num) => num,
   Err(_) => break,
};



println!("Input your second digit (Cannot be decimal or negative)");

io::stdin()
    .read_line(&mut num2)
    .expect("Invalid Number");

let num2: u32 = match num2.trim().parse() {
    Ok(num) => num,
    Err(_) => break,
};

println!("{num1} / {num2} =");
println!("{}", num1 / &num2);

let mut equate = String::new();

println!("Select an equation type: Add, Subtract, Multiply, Divide, (Case Sensitive: use words exactly as written)");
equate.clear();

io::stdin()
   .read_line(&mut equate)
   .expect("Input should match the given words.");
match equate.trim() {
    "Add" => add(),
    "Subtract" => sub(),
    "Multiply" => mult(),
    "Divide" => div(),
    _ => panic!("Your input should match the given words."),
    }
  }
}