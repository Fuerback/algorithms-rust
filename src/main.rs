use std::env;
use rand::Rng;

#[macro_use]
extern crate rocket;

#[get("/binary-search/<number>")]
fn binary(number: &str) -> &'static str {
    let number = number.parse::<i32>().unwrap();
    return binary_search(number);
}

#[get("/quicksort")]
fn quicksort() -> String {
    let mut elements = Vec::new();
    let mut i = 0;
    let mut rng = rand::thread_rng();

    while i < 50 {
        elements.push(rng.gen::<i32>());
        i = i + 1;
    }

    let sorted_elements = quick_sort(&mut elements);
    format!("Here are the sorted elements {:?}", sorted_elements)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![binary, quicksort])
}

fn binary_search(num: i32) -> &'static str {
    let mut numbers_vec = vec![5, 7, 11, 48, 53, 63, 92, 102, 123, 124, 142, 161, 162, 167, 191, 228, 268, 274, 300, 306, 367, 410, 428, 438, 465, 466, 491, 512, 514, 549, 568, 593, 631, 633, 668, 670, 702, 718, 743, 756, 771, 780, 800, 802, 834, 875, 911, 926, 930, 939];

    let result = numbers_vec.binary_search(&num);
    if result.is_ok() {
        return "found"
    }

    return "not found"
}

fn quick_sort(slice: &mut [i32]) -> Vec<i32> {
    if !slice.is_empty() {
        let partition_index = partition(slice);
        let len = slice.len();

        quick_sort(&mut slice[0..partition_index]);
        quick_sort(&mut slice[partition_index + 1..len]);
        assert_sorted(slice);
        return slice.to_vec()
    }

    return slice.to_vec()
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