mod binary_search;
mod compare_strings;
mod quicksort_algorithm;

use crate::binary_search::binary_search;
use crate::compare_strings::{compare_strings, Matches};
use crate::quicksort_algorithm::quick_sort;
use rand::Rng;
use rocket::serde::json::Json;

#[macro_use]
extern crate rocket;

#[get("/binary-search/<number>")]
fn binary(number: &str) -> &'static str {
    let number = number.parse::<i32>().unwrap();
    return binary_search(number);
}

#[get("/compare-strings")]
fn compare() -> Json<Matches> {
    Json(compare_strings())
}

#[get("/quicksort")]
fn quicksort() -> String {
    let mut elements = Vec::new();
    let mut i = 0;
    let mut rng = rand::thread_rng();

    while i < 50 {
        elements.push(rng.gen::<i32>());
        i += 1;
    }

    let sorted_elements = quick_sort(&mut elements);
    format!("Here are the sorted elements {:?}", sorted_elements)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![binary, quicksort, compare])
}
