use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    //guessing game
    // let secretnumber = rand::thread_rng().gen_range(1..101);
    // println!("The secret number is {}", secretnumber);
    // loop {
    //     let mut guess = String::new();
    //     io::stdin().read_line(&mut guess).expect("cannot read line");
    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };
    //     match guess.cmp(&secretnumber) {
    //         Ordering::Less => println!("too low"),
    //         Ordering::Greater => println!("too high"),
    //         Ordering::Equal => {
    //             println!("you win!");
    //             break;`
    //         }
    //     }
    // }

    //index
    // let a = [1, 2, 3, 4, 5];
    // println!("{:?}", a);

    // let mut index = String::new();
    // io::stdin().read_line(&mut index).expect("cannot read line");
    // let index: usize = index.trim().parse().expect("couldnt");
    // let element = a[index];
    // println!("{}", element);

    // another_function(5);

    // print_labeled_measurement(5, 'h');

    // let y = {
    //     let x = 3;
    //     x + 1
    // };

    // println!("The value of y is: {}", y);

    // let x = five();

    // println!("The value of x is: {}", x);

    // let number = 3;

    // if number < 5 {
    //     println!("condition was true");
    // } else {
    //     println!("condition was false");
    // }

    // let number = 6;

    // if number % 4 == 0 {
    //     println!("number is divisible by 4");
    // } else if number % 3 == 0 {
    //     println!("number is divisible by 3");
    // } else if number % 2 == 0 {
    //     println!("number is divisible by 2");
    // } else {
    //     println!("number is not divisible by 4, 3, or 2");
    // }

    //loops

    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {}", count);
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {}", remaining);
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }

    //     count += 1;
    // }
    // println!("End count = {}", count);

    // let mut counter = 0;

    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // println!("The result is {}", result);

    //while loop
    // let mut number = 3;
    // while number != 0 {
    //     println!("{}", number);
    //     number -= 1;
    // }
    // println!("LIFTOFF");

    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;
    // while index < 5 {
    //     println!("{}", a[index]);
    //     index += 1
    // }

    // for element in a {
    //     println!("{}", element)
    // }

    // for number in (1..4).rev() {
    //     println!("{}", number)
    // }
    // println!("LIFTOFF")

    // let string = "fool";
    // println!("{:?}", string.as_bytes());
}

//functions

// fn another_function(x: i32) {
//     println!("The value of x is {}", x);
// }
// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {}{}", value, unit_label);
// }
// fn five() -> i32 {
//     5
// }
