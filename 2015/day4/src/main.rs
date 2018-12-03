extern crate md5;

fn find_hash(salt: &str, pattern: &str) -> u32 {
    let mut matching_number: u32 = 0;

    for i in 1.. {
        let hash = format!("{:x}", md5::compute(format!("{}{}", salt, i))).to_string();
        if hash.starts_with(pattern) {
            matching_number = i;
            break;
        }
    }

    matching_number
}

fn main() {
    assert_eq!(find_hash("abcdef", "00000"), 609043);
    assert_eq!(find_hash("pqrstuv", "00000"), 1048970);

    println!("Part one: {}", find_hash("yzbqklnj", "00000"));
    println!("Part two: {}", find_hash("yzbqklnj", "000000"));
}
