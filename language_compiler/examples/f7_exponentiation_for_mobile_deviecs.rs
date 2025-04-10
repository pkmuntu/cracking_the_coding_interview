fn quick_pow_rec(base: f64, power: i32) -> f64 {
    if power == 0 {
        return 1.0;
    }

    let half = quick_pow_rec(base, power / 2);

    if power % 2 == 0 {
        return half * half;
    } else {
        return half * half * base;
    }
}

fn quick_pow(b: f64, p: i32) -> f64 {
    let mut base = b;
    let mut power = p;

    if power < 0 {
        let d: f64 = 1.0;
        base = d / base;
        power = -power;
    }
    return quick_pow_rec(base, power);
}

fn main() {
    let base: f64 = 2.0;
    let power = 4;

    let result = quick_pow(base, power);

    println!("{:}", result);
}

// Time complexity = O(log n)
// Space complexity = O(log n)
