// ユーザ入力
use std::io;
// Rng (random number generators)
use rand::Rng;

fn main() {
    println!("Guess the number!!");

    // 長い変数名は"_"でつなぐ
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    println!("Please input in your guess.");

    // 型推論(mutable)
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); // error handling

    // printf("%s-hoge", hoge)みたいな感じ
    println!("You guessed: {}", guess);
}
