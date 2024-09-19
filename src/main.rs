fn check_guess(guess: i32, secret: i32) -> i32 {
    // return 0 if guess is correct
    // return 1 if guess is too high
    // return -1 if guess is too low
    if guess == secret {
        0
    }
    else if guess > secret {
        1
    }
    else {
        -1
    }
}

fn main() {
    loop {
        // Mut to simulate user input
        let mut guess:i32 = 47;
        let secret = 47;
        let mut result = check_guess(guess, secret);
        if result == 1 {
            println!("Too High!");
        }
        if result == -1 {
            println!("Too Low!")
        }
        if result == 0 {
            println!("Correct!");
            break
        }
    }
}
