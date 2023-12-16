
pub fn input_as_lines(s: &str) -> Vec<String> {
    s.split("\r\n").map(|x| x.to_owned()).collect::<Vec<_>>()
}

pub fn input_as_ints(s: &str) -> Vec<i64> {
    let input_lines = input_as_lines(s);

    input_lines.iter()
        .map(|s| s.parse::<i64>().expect("Unable to convert line to i64"))
        .collect()
}


pub fn lcm(nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}  