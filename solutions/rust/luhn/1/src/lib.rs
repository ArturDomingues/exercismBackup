/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut digits = Vec::new();
    for c in code.chars() {
        if c == ' ' {
            continue;
        }
        match c.to_digit(10) {
            Some(d) => digits.push(d),
            None => return false, // invalid character
        }
    }
    if digits.len() < 2 {
        return false;
    }
    let luhn_sum: u32 = digits
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &d)| {
            if i % 2 == 1 {
                let dbl = d * 2;
                if dbl > 9 { dbl - 9 } else { dbl }
            } else {
                d
            }
        })
        .sum();
    luhn_sum % 10 == 0
}
