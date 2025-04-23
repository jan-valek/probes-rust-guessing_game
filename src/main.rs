use std::{io, cmp::Ordering };
use rand::{thread_rng, Rng};

fn main() {
    println!("Guessing game!");
    let secret_number = thread_rng().gen_range(1..=100);
    let mut number_of_guesses:Vec<i32> = Vec::new();
    
    loop{
        println!("Enter your guess:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Nelze nacist zdrojova data.");
        
        let guess: &str = guess.trim();

        if guess.to_lowercase() == "q" {
            println!("Ukoncil jste hru pri naledujicim poctu pokusu:{}", number_of_guesses.len());
            break;
        }        

        let guess:i32 = match guess.parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Nepovedlose převést vstup na číslo, z důvodu: '{}'", err);
                continue;
            }
        };

        number_of_guesses.insert(number_of_guesses.len(), guess);

        match guess.cmp(&secret_number) {                        
            Ordering::Less => println!("zvyšte tipované číslo!"),
            Ordering::Greater => println!("snižte tipované číslo!"),
            Ordering::Equal => {
                println!("Zásah");
                print!("Statistika: počet pokusů '{}', zadané tipy:'",number_of_guesses.len());
                for (index,value) in number_of_guesses.iter().enumerate() {
                    if index > 0 {
                        print!(" ");
                    }
                    
                    print!("{}",value);
                    
                    if index < number_of_guesses.len()-1 {
                        print!(",");
                    }
                }
                println!("',");
                break;
            }
        }
    }
}
