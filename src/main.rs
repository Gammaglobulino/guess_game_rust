use rand::Rng;
use std::io;

#[derive(Debug)]
struct User {
    name:String,
    last_name:String
}

impl User{
    fn new() -> User{
        User{name:String::from(""),last_name:String::from("")}
    }
    
}

fn main() {

    println!("Guess a number:");
    let mut usr=User::new();
    println!("Please enter your name:");
    io::stdin().read_line(&mut usr.name).expect("Error reading input");
    println!("Please enter your last name:");
    io::stdin().read_line(&mut usr.last_name).expect("Error reading input");
    println!("Hello > {}-{} :",usr.name,usr.last_name);
    
    let secret=rand::thread_rng().gen_range(1,100);
    let mut attempt=0;
    loop{
            println!("Please enter a number :>");
            let mut guessed_num=String::new();
            match io::stdin().read_line(&mut guessed_num){
                Ok(_) => {},
                Err(n) =>println!("{}",n)

            }

            let guess=guessed_num.trim().parse().expect("Error parsing the string");
            if secret==guess{
                println!("guessed ok secret was {}\n bye {}{}",secret,usr.name,usr.last_name);
                break;
            }else{
                if guess>secret{
                    println!("The secret number is lower");
                }else{
                    println!("The secret number is higher");
                }

            }
            attempt+=1;
            
        }
        println!("Numero tentativi {}",attempt);

        
    }

