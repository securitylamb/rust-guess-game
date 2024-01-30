use std::cmp::Ordering;
use std::io;
use rand::Rng;
fn main() {
    println!("Guess the number!");


  loop{
    println!("Please input your guess.");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guess = String::new();
    // the main function of the readline() method is to read user input push /append to the string passed to it so it is very nessary to make the input string
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    // here i shadowed previously declared guess variable
    // or i can say i converted one type to another
    // let guess:u32 = guess.trim().parse().expect("pls type a number");
    // expect will crash the program if we didnot recevive a number so instead of crashing the program i will do this 
    let guess:u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };
    //  trim method also removes /n and /r that could enter the string when we hit enter 
    // parse method od string convert the string to another type
    println!("The secret number is: {secret_number}");
  
      println!("you guessed: {guess}");
    
      match guess.cmp(&secret_number){
          // this can also be called arms for match it will check for greator , less ,equal arm 
        Ordering::Less => println!("Too small !!"),
        Ordering::Greater => println!("Too big !!"),
        Ordering::Equal => {
          println!("You WIN!!!");
          break;
        },
        
      }
  
    }
   

  }
