// Rng 乱数生成(random number generators)
use rand::Rng;
// io input/output
use std::io;
// cmp compare
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!!");

    // 長い変数名はアンスコ
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input in your guess.");

        // 型推論(mutable)
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess) // read_line method on the standard input handle to get input from the user. 
            .expect("Failed to read line"); // error handling
        // String型をスペース削除 && uint32型変換(ParseInt)
        let guess: u32 = guess.trim().parse().expect("Please Type Number!!");

        // printf("%s-hoge", hoge)みたいな感じ
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!!"),
            Ordering::Greater => println!("Too big!!"),
            Ordering::Equal => {
                // 正解時処理を抜ける
                println!("You win!!");
                break;
            },
        }
    }
}
