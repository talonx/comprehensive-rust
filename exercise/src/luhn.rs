use core::fmt::Debug;

unsafe fn sum_of_digits(input: u32) -> u32 {
    let mut rem: u32 = input % 10;
    let mut rest: u32 = input / 10;
    let mut sum = rem;
    while rest > 0 {
        rem = rest % 10;
        sum += rem;
        rest = rest/10;
    }
    sum += rest;
    sum
}

fn debug<D: Debug>(msg: D) {
    println!("{:?}", msg);
}

fn luhn(cc_number: &str) -> bool {
    let c_iter = cc_number.chars().rev();
    let mut ind = 0;
    let mut sum = 0;
    for c in c_iter {
        if c != ' ' {
            let dig = c.to_digit(10);
            debug(dig);
            if dig != None {
                let mut num = dig.unwrap();
                ind += 1;
                if ind % 2 == 0 {
                    debug(format!("Doubling {}", num));
                    num = num * 2;
                    if num > 9 {
                        unsafe {
                            num = sum_of_digits(num);
                        }
                    }
                }
                debug(format!("Final num {}", num));
                sum += num;
            } else {
                return false
            }
        }
    }
    debug(format!("Sum {}", sum));
    if ind > 1 {
        sum % 10 == 0
    } else {
        false
    }
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    //assert!(luhn("4539 3195 0343 6467"));
    //assert!(luhn("7992 7398 713"));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
    assert!(!luhn("foo 0 0"));
}

//fn main() {
    //luhn("2349 0923 0932 1412");
//}
