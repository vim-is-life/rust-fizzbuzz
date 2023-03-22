/*
 * count from 1 to 100
 * any number divisible by 3 is turned to fizz
 * any number divisible by 5 is turned to buzz
 * any number divisible by 3 and 5 is fizz buzz
 */

fn fizz_buzz(ceil: i32) {
    let range = 1..=ceil;
    range.for_each(|x| match (x % 3, x % 5) {
        (0, 0) => println!("fizzbuzz"),
        (0, _) => println!("fizz"),
        (_, 0) => println!("buzz"),
        (_, _) => println!("{x}"),
    });
}

fn main() {
    println!("Please input your ceiling for the fizzbuzz program");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let upper_lim: i32 = input.trim().parse().unwrap();
    println!("Running fizz buzz... ");

    fizz_buzz(upper_lim);
}
