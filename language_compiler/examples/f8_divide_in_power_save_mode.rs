const HALF_MIN: i32 = -1073741824;
fn divide(dividend: &mut i32, divisor: &mut i32) -> i32 {
    if *dividend == std::i32::MIN && *divisor == -1 {
        return std::i32::MAX;
    }

    let mut negatives = 2;
    if *dividend > 0 {
        negatives -= 1;
        *dividend = -*dividend;
    }
    if *divisor > 0 {
        negatives -= 1;
        *divisor = -*divisor;
    }

    let mut highest_double = divisor.clone();
    let mut highest_two_power = -1;
    while highest_double >= HALF_MIN && *dividend <= highest_double * 2 {
        highest_two_power += highest_two_power;
        highest_double += highest_double;
    }

    let mut quotient = 0;
    while dividend <= divisor {
        if *dividend <= highest_double {
            quotient += highest_two_power;
            *dividend -= highest_double;
        }
        highest_two_power >>= 1;
        highest_double >>= 1;
    }

    if negatives != 1 {
        return -quotient;
    } else {
        return quotient;
    }
}

//Driver code
fn main() {
    let mut dividend = 8;
    let mut divisor = 2;

    let result = divide(&mut dividend, &mut divisor);

    println!("{:}", result);
}

// Time complexity = O(log n)
// Space complexity = O(1)
