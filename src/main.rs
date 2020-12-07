use rand::Rng;
use std::io;

fn main() {
    println!("Guess a number:");
    let secret=rand::thread_rng().gen_range(1,100);
    
    loop{
            println!("Please enter a number :>");
            let mut guessed_num=String::new();
            io::stdin().read_line(&mut guessed_num).expect("Error reading input");
            let guess=guessed_num.trim().parse().expect("Error parsing the string");
            if secret==guess{
                println!("guessed ok secret was {}",secret);
                break;
            }else{
                if guess>secret{
                    println!("The secret number is lower");
                }else{
                    println!("The secret number is higher");
                }

            }
            
        }

        
    }

