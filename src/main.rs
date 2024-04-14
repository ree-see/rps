use std::collections::HashMap;
use std::io;

fn main() {
    let mut score: HashMap<String, u32> = HashMap::new();
    score.insert(
        "Player".to_string(), 0
    );
    score.insert(
        "CPU".to_string(), 0
    );

    let player_score = score.get("Player").unwrap();
    let cpu_score = score.get("CPU").unwrap();
    
    println!("Player: {:?} CPU: {:?}", player_score, cpu_score);

    let mut input = String::new();

    println!("Would you like to play rock, paper, scissors? y/n");
    // match io::stdin().read_line(&mut input) {
    //     Ok(_) => { 
    //         if input == "y".to_string() { 
    //             println!("Rock...Paper...Scissor...Shoot!") 
    //         } 
    //     },
    //     Err(e) => { println!("Something went wrong\n{}", e) },
    // }
    io::stdin()
        .read_line(&mut input)
        .expect("something went wrong");
    
    // if input == "y" {
    //     println!("you would like to play");
    // } else {
    //     print!("Goodbye");
    // }
    let y: String = String::from("y");
    if input == y {
        print!("input is y")
    }

}
