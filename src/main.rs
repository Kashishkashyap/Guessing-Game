use std::io;
use rand::prelude::*;
fn main() {
    let guess_list=["apple","mango","grapes","pineapple","banana","orange"];
    let mut rng= thread_rng();
    let index= rng.gen_range(0..guess_list.len());
    let guessing_fruit= guess_list[index];
    println!("Random fuit to be guesses is: {}", guessing_fruit);
    println!("Make a guess: ");
    loop{
        let mut input= String::new();
        match io::stdin().read_line(&mut input){
            Ok(_)=>{
                let fruit_selected= input.trim().to_lowercase();
                println!("Selected fruit: {}",fruit_selected);
                if !guess_list.contains(&fruit_selected.as_str()){
                    println!("Fruit does not exists");
                    continue;
                }
                if guess_checker(&fruit_selected, guessing_fruit){
                    println!("Winner");
                    break;
                }else{
                    println!("Retry");
                }
            }
            Err(error)=>{
                println!("Error: {}", error);
            }
        }   
    }
}
fn guess_checker(fruit_selected: &str, guessing_fruit: &str)->bool{
    return fruit_selected == guessing_fruit;
}