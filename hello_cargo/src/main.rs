// ユーザ入力
use std::io;

fn main() {
    println!("Guess the number!!");
    println!("Please input in your guess.");

    // 型推論(mutable)
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); // error handling

    // printf("%s-hoge", hoge)みたいな感じ
    println!("You guess:{}", guess);
}
