use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    //生成随机数
    let random = rand::thread_rng().gen_range(1..=100);

    loop {
        //输入变量是一个String
        let mut input: String = String::new();
        //接受输入
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let guess: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {guess}");
        match guess.cmp(&random) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                print!("You win!");
                break;
            }
        }
    }
}
