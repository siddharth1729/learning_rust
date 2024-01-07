use std::io;

pub fn guessing_game() {
    println!(
        "\n

    ██      ███████  ██████  ██  ██████  ███    ██ 
    ██      ██      ██       ██ ██    ██ ████   ██ 
    ██      █████   ██   ███ ██ ██    ██ ██ ██  ██ 
    ██      ██      ██    ██ ██ ██    ██ ██  ██ ██ 
    ███████ ███████  ██████  ██  ██████  ██   ████ 
                                                  
                                                  \n"
    );
    println!("Please enter a number:");

    let mut input = String::new();
    let mut guss = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    io::stdin()
        .read_line(&mut guss)
        .expect("Failed to read guss");
    // Parse the input into a u32
    let guessed_num: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return; // or handle the error as needed
        }
    };

    // Now you can use guessed_num as a u32
    println!("You guessed: {}", guessed_num);
}
