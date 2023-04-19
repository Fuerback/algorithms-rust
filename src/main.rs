use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some((_, tail)) = args.split_first() {
        let mut elements: Vec<i32> = tail.iter().map(|x| x.parse::<i32>().unwrap()).collect();
        quick_sort(&mut elements);
        println!("{:?}", elements);
    }
}

fn quick_sort(slice: &mut [i32]) {
    if !slice.is_empty() {
        let partition_index = partition(slice);
        let len = slice.len();

        quick_sort(&mut slice[0..partition_index]);
        quick_sort(&mut slice[partition_index + 1..len]);
        assert_sorted(slice);
    }
}

fn partition(slice: &mut [i32]) -> usize {
    let len = slice.len();
    let pivot = slice[len - 1];
    let mut i = 0;
    let mut j = 0;

    while j < len - 1 {
        if slice[j] <= pivot {
            slice.swap(i, j);
            i += 1;
        }
        j += 1;
    }

    slice.swap(i, len - 1);

    i
}

fn assert_sorted(slice: &[i32]) {
    for i in 1..slice.len() {
        assert!(slice[i - 1] <= slice[i])
    }
}

fn binary_search(num: i32, numbers: Vec<i32>) {
    // let args: Vec<String> = env::args().collect();
    //
    // let number_arg = &args[1];
    // let number = number_arg.parse::<i32>().unwrap();
    //
    // println!("Number is {}", number);
    //
    // let mut numbers_vec = vec![5, 7, 11, 48, 53, 63, 92, 102, 123, 124, 142, 161, 162, 167, 191, 228, 268, 274, 300, 306, 367, 410, 428, 438, 465, 466, 491, 512, 514, 549, 568, 593, 631, 633, 668, 670, 702, 718, 743, 756, 771, 780, 800, 802, 834, 875, 911, 926, 930, 939];
    // binary_search(number, numbers_vec);

    let result = numbers.binary_search(&num);
    if result.is_ok() {
        println!("found")
    } else {
        println!("not found")
    }
}
