pub fn binary_search(num: i32) -> &'static str {
    let numbers_vec = vec![5, 7, 11, 48, 53, 63, 92, 102, 123, 124, 142, 161, 162, 167, 191, 228, 268, 274, 300, 306, 367, 410, 428, 438, 465, 466, 491, 512, 514, 549, 568, 593, 631, 633, 668, 670, 702, 718, 743, 756, 771, 780, 800, 802, 834, 875, 911, 926, 930, 939];

    let result = numbers_vec.binary_search(&num);
    if result.is_ok() {
        return "found"
    }

    return "not found"
}