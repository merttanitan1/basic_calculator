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
            add();
        }
        else if opr == 2 {
            sub();
        }
        else if opr == 3 {
            divide();
        }
        else if opr == 4 {
            multiply();
        }
        else {
            break;
        }
    }
}

fn add(){
    loop{
        let mut a = String::new();
        let mut b = String::new();
        println!("Please enter first num.");
        io::stdin().read_line(&mut a).expect("Failed to read line");
        println!("Please enter second num.");
        io::stdin().read_line(&mut b).expect("Failed to read line");
        let a: i32 = match a.trim().parse(){Ok(num)=>num, Err(_) => continue,};
        let b: i32 = match b.trim().parse(){Ok(num)=>num, Err(_) => continue,};
        println!("Result: {}", a + b);
        break;
    }
}

fn sub(){
    loop{
        let mut a = String::new();
        let mut b = String::new();
        println!("Please enter first num.");
        io::stdin().read_line(&mut a).expect("Failed to read line");
        println!("Please enter second num.");
        io::stdin().read_line(&mut b).expect("Failed to read line");
        let a: i32 = match a.trim().parse(){Ok(num)=>num, Err(_) => continue,};
        let b: i32 = match b.trim().parse(){Ok(num)=>num, Err(_) => continue,};
        println!("Result: {}", a - b);
        break;
    }
}

fn divide(){
    loop{
        let mut a = String::new();
        let mut b = String::new();
        println!("Please enter first num.");
        io::stdin().read_line(&mut a).expect("Failed to read line");
        println!("Please enter second num.");
        io::stdin().read_line(&mut b).expect("Failed to read line");
        let a: f32 = match a.trim().parse(){Ok(num)=>num, Err(_) => continue,};
        let b: f32 = match b.trim().parse(){Ok(num)=>num, Err(_) => continue,};
        println!("Result: {}", a / b);
        break;
    }
}

fn multiply(){
    loop{
        let mut a = String::new();
        let mut b = String::new();
        println!("Please enter first num.");
        io::stdin().read_line(&mut a).expect("Failed to read line");
        println!("Please enter second num.");
        io::stdin().read_line(&mut b).expect("Failed to read line");
        let a: i32 = match a.trim().parse(){Ok(num)=>num, Err(_) => continue,};
        let b: i32 = match b.trim().parse(){Ok(num)=>num, Err(_) => continue,};
        println!("Result: {}", a * b);
        break;
    }
}