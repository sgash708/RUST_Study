// constは定数
const MAX_POINTS: u32 = 100_000;

fn main() {
    // mut は後で変更するものに対しては必ず必要
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    println!("{}", MAX_POINTS);
}
