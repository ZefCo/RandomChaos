pub fn run() {
    // seed(100, 1);
    iterator(3);
}

// fn seed(iter: i64, x_o: i64) {
//     // let x: i64 = x_o;
//     for x in x_o..=iter{
//         // println!("{}: {}", )
//         if x % 15 == 0 {
//             println!("Fizzbuzz!")
//         } else if x % 5 == 0 {
//             println!("Buzz!")
//         } else if x % 3 == 0 {
//             println!("Fizz!")
//         } else {
//             println!("")
//         }
//     }
// }

// fn iterator(iter: i64, x_o: i64) {
//     for x in x_o..=iter {
//         // println!("Sending {} to fizzbuzz", x);
//         fizzbuzz(x);
//     }
// }

// fn fizzbuzz(input: i64) {
//     if input % 15 == 0 {
//         println!("{}: Fizzbuzz!", input);
//     } else if input % 5 == 0 {
//         // println!("Buzz!")
//         // let output = "Buzz!"
//         println!("{}: Buzz!", input);
//     } else if input % 3 == 0 {
//         // println!("Fizz!")
//         // let output = "Fizz!"
//         println!("{}: Fizz!", input);
//     } else {
//         // println!("")
//         // let output = ""
//         println!("{}: -_-", input);
//     }
// }

fn iterator(iter: u64) {
    let mut index = 0;
    let mut x = 1.1;
    while index <= iter {
        x = fofx(3.8, x);
        index += 1
    }
    println!("x ended up being {}", x)
}

fn fofx(b:f64, x:f64) -> f64 {
    // b * x * ( 1.0 - x )
    b * x * (1.0 + x)
}