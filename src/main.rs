use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let number_arg = &args[1];
    let number = number_arg.parse::<i32>().unwrap();

    println!("Number is {}", number);

    let mut numbers_vec = vec![5, 7, 11, 48, 53, 63, 92, 102, 123, 124, 142, 161, 162, 167, 191, 228, 268, 274, 300, 306, 367, 410, 428, 438, 465, 466, 491, 512, 514, 549, 568, 593, 631, 633, 668, 670, 702, 718, 743, 756, 771, 780, 800, 802, 834, 875, 911, 926, 930, 939];
    binary_search(number, numbers_vec);
}

fn binary_search(num: i32, numbers: Vec<i32>) {
    let result = numbers.binary_search(&num);
    if result.is_ok() {
        println!("found")
    } else {
        println!("not found")
    }
}
