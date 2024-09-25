use std::io;

fn main() {
    loop{
        let mut opr = String::new();
        println!("Please select the operation number you will do:");
        println!("1. Add");
        println!("2. Sub");
        println!("3. Divide");
        println!("4. Multiply");
        println!("5. Exit");
        io::stdin()
            .read_line(&mut opr)
            .expect("Failed to read line");
        let opr: u32 = match opr.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if opr == 1 {
            let mut a = String::new();
            let mut b = String::new();
            println!("Please enter first num.");
            io::stdin().read_line(&mut a).expect("Failed to read line");
            println!("Please enter second num.");
            io::stdin().read_line(&mut b).expect("Failed to read line");
            let a: i32 = match a.trim().parse(){Ok(num)=>num, Err(_) => continue,};
            let b: i32 = match b.trim().parse(){Ok(num)=>num, Err(_) => continue,};
            println!("Result: {}", add(a, b));
        }
        else if opr == 2 {
            let mut a = String::new();
            let mut b = String::new();
            println!("Please enter first num.");
            io::stdin().read_line(&mut a).expect("Failed to read line");
            println!("Please enter second num.");
            io::stdin().read_line(&mut b).expect("Failed to read line");
            let a: i32 = match a.trim().parse(){Ok(num)=>num, Err(_) => continue,};
            let b: i32 = match b.trim().parse(){Ok(num)=>num, Err(_) => continue,};
            println!("Result: {}", sub(a, b));
        }
        else if opr == 3 {
            let mut a = String::new();
            let mut b = String::new();
            println!("Please enter first num.");
            io::stdin().read_line(&mut a).expect("Failed to read line");
            println!("Please enter second num.");
            io::stdin().read_line(&mut b).expect("Failed to read line");
            let a: f32 = match a.trim().parse(){Ok(num)=>num, Err(_) => continue,};
            let b: f32 = match b.trim().parse(){Ok(num)=>num, Err(_) => continue,};
            println!("Result: {}", divide(a, b));
        }
        else if opr == 4 {
            let mut a = String::new();
            let mut b = String::new();
            println!("Please enter first num.");
            io::stdin().read_line(&mut a).expect("Failed to read line");
            println!("Please enter second num.");
            io::stdin().read_line(&mut b).expect("Failed to read line");
            let a: i32 = match a.trim().parse(){Ok(num)=>num, Err(_) => continue,};
            let b: i32 = match b.trim().parse(){Ok(num)=>num, Err(_) => continue,};
            println!("Result: {}", multiply(a, b));
        }
        else {
            break;
        }
    }
}

fn add(a: i32, b: i32) -> i32{
    a + b
}

fn sub(a: i32, b: i32) -> i32{
    a - b
}

fn divide(a: f32, b: f32) -> f32{
    a / b
}

fn multiply(a: i32, b:i32) -> i32{
    a * b
}