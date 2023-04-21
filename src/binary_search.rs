pub fn binary_search(num: i32) -> &'static str {
    let elements = vec![5, 7, 11, 48, 53, 63, 92, 102, 123, 124, 142, 161, 162, 167, 191, 228, 268, 274, 300, 306, 367, 410, 428, 438, 465, 466, 491, 512, 514, 549, 568, 593, 631, 633, 668, 670, 702, 718, 743, 756, 771, 780, 800, 802, 834, 875, 911, 926, 930, 939];

    //let result = numbers_vec.binary_search(&num); RUST CHEAT!!
    let result = find_number(num, elements);
    if result {
        return "found"
    }

    return "not found"
}

fn find_number(num: i32, elements: Vec<i32>) -> bool {
    let size = elements.len();
    let middle = size/2;
    let middle_value = elements[middle];

    if middle_value.eq(&num) {
        return true
    };
    if size == 1 {
        return false
    };

    let (left, right) = elements.split_at(middle);
    if middle_value > num {
        let new_elements = left.to_vec();
        return find_number(num, new_elements);
    };

    let new_elements = right.to_vec();
    return find_number(num, new_elements);
}