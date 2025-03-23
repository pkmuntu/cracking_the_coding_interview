fn ones(fare: i32) -> String {
    match fare {
        1 => return "One".to_string(),
        2 => return "Two".to_string(),
        3 => return "Three".to_string(),
        4 => return "Four".to_string(),
        5 => return "Five".to_string(),
        6 => return "Six".to_string(),
        7 => return "Seven".to_string(),
        8 => return "Eight".to_string(),
        9 => return "Nine".to_string(),
        _ => return "".to_string(),
    }
}

fn teens(fare: i32) -> String {
    match fare {
        10 => return "Ten".to_string(),
        11 => return "Eleven".to_string(),
        12 => return "Twelve".to_string(),
        13 => return "Thirteen".to_string(),
        14 => return "Fourteen".to_string(),
        15 => return "Fifteen".to_string(),
        16 => return "Sixteen".to_string(),
        17 => return "Seventeen".to_string(),
        18 => return "Eighteen".to_string(),
        19 => return "Nineteen".to_string(),
        _ => return "".to_string(),
    }
}
fn tens(fare: i32) -> String {
    match fare {
        2 => return "Twenty".to_string(),
        3 => return "Thirty".to_string(),
        4 => return "Forty".to_string(),
        5 => return "Fifty".to_string(),
        6 => return "Sixty".to_string(),
        7 => return "Seventy".to_string(),
        8 => return "Eighty".to_string(),
        9 => return "Ninety".to_string(),
        _ => return "".to_string(),
    }
}

fn two(fare: i32) -> String {
    if fare == 0 {
        return "".to_string();
    } else if fare < 10 {
        return ones(fare);
    } else if fare < 20 {
        return teens(fare);
    } else {
        let tenner = fare / 10;
        let rest = fare - tenner * 10;
        if rest != 0 {
            let t = format!("{}{}{}", tens(tenner), " ", ones(rest));
            return t;
        } else {
            return tens(tenner);
        }
    }
}

fn three(fare: i32) -> String {
    let hundred = fare / 100;
    let rest = fare - hundred * 100;
    let mut res = "".to_string();
    let mut r = String::new();
    if hundred * rest != 0 {
        r = format!("{}{}{}", ones(hundred), " Hundred ", two(rest));
        res = r;
    } else if (hundred == 0) && (rest != 0) {
        res = two(rest);
    } else if (hundred != 0) && (rest == 0) {
        res = ones(hundred);
        res.push_str(" Hundred");
    }

    return res;
}

fn fare_in_words(fare: i32) -> String {
    if fare == 0 {
        return "Zero".to_string();
    }

    let billion = fare / 1000000000;
    let million = (fare - billion * 1000000000) / 1000000;
    let thousand = (fare - billion * 1000000000 - million * 1000000) / 1000;
    let rest = fare - billion * 1000000000 - million * 1000000 - thousand * 1000;
    let mut result: String = "".to_string();

    if billion != 0 {
        result = three(billion);
        result.push_str(" Billion");
    }
    if million != 0 {
        if !result.is_empty() {
            result.push_str(" ");
        }
        result.push_str(&three(million));
        result.push_str(" Million");
    }
    if thousand != 0 {
        if !result.is_empty() {
            result.push_str(" ");
        }
        result.push_str(&three(thousand));
        result.push_str(" Thousand");
    }
    if rest != 0 {
        if !result.is_empty() {
            result.push_str(" ");
        }
        result.push_str(&three(rest));
    }
    return result;
}

fn main() {
    // Driver code

    let fare = 5648;
    println!("{}{}{}", "The fare is: ", fare_in_words(fare), " dollars");
}

// Time complexity = O(n)
// Space complexity = O(1)
