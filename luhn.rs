pub fn luhn(cc_number: &str) -> bool {
    if cc_number.trim().is_empty() {
        return false;
    }

    if cc_number.chars().any(|c| !c.is_digit(10) && c != ' ') {
        return false;
    }


    let mut sum = 0;
    let mut double = false;

    for c in cc_number.chars().rev() {
        if let Some(digit) = c.to_digit(10) {
            if double {
                let double_digit = digit * 2;
                sum +=
                    if double_digit > 9 { double_digit - 9 } else { double_digit };
            } else {
                sum += digit;
            }
            double = !double;
        } else {
            continue;
        }
    }

    sum % 10 == 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_cc_number() {
        assert!(luhn("4263 9826 4026 9299"));
        assert!(luhn("4539 3195 0343 6467"));
        assert!(luhn("7992 7398 713"));
        assert!(luhn("4242 4242 4242 4242"));
        assert!(luhn("0000 0000 0000 0000"));
    }

    #[test]
    fn test_invalid_cc_number() {
        assert!(!luhn(""));
        assert!(!luhn(" "));
        assert!(!luhn("abcd"));
        assert!(!luhn("4223 9826 4026 92999"));
        assert!(!luhn("4223 9826 4026 9299"));
        assert!(!luhn("4539 3195 0343 6476"));
        assert!(!luhn("8273 1232 7352 0569"));
    }
}