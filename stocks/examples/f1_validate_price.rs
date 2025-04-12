#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum STATE {
    START,
    INTEGER,
    DECIMAL,
    UNKNOWN,
    AFTERDECIMAL,
}

fn get_next_state(current_state: STATE, ch: char) -> STATE {
    match current_state {
        STATE::START => {
            if ch == '.' {
                return STATE::DECIMAL;
            } else if ch >= '0' && ch <= '9' {
                return STATE::INTEGER;
            } else {
                return STATE::UNKNOWN;
            }
        }
        STATE::INTEGER => {
            if ch == '.' {
                return STATE::DECIMAL;
            } else if ch >= '0' && ch <= '9' {
                return STATE::INTEGER;
            } else {
                return STATE::UNKNOWN;
            }
        }
        STATE::DECIMAL => {
            if ch >= '0' && ch <= '9' {
                return STATE::AFTERDECIMAL;
            } else {
                return STATE::UNKNOWN;
            }
        }
        STATE::UNKNOWN => return STATE::UNKNOWN,
        STATE::AFTERDECIMAL => {
            if ch >= '0' && ch <= '9' {
                return STATE::AFTERDECIMAL;
            } else {
                return STATE::UNKNOWN;
            }
        }
    };
}

fn is_price_valid(s: String) -> bool {
    if s.is_empty() {
        return true;
    }

    let mut i = 0;
    if s.chars().nth(i).unwrap() == '+' || s.chars().nth(i).unwrap() == '-' {
        i += 1;

        let mut current_state = STATE::START;
        while i < s.len() {
            current_state = get_next_state(current_state, s.chars().nth(i).unwrap());

            if current_state == STATE::UNKNOWN {
                return false;
            }

            i = i + 1;
        }

        if current_state == STATE::DECIMAL {
            return false;
        }
        return true;
    }
    return false;
}

fn main() {
    println!(
        "{}{:}",
        "Is the price valid +40.325? ",
        is_price_valid("+40.325".to_string())
    );
    println!(
        "{}{:}",
        "Is the price valid -1.1.1? ",
        is_price_valid("-1.1.1".to_string())
    );
    println!(
        "{}{:}",
        "Is the price valid -222? ",
        is_price_valid("-222".to_string())
    );
    println!(
        "{}{:}",
        "Is the price valid ++22? ",
        is_price_valid("++22".to_string())
    );
    println!(
        "{}{:}",
        "Is the price valid 10.1? ",
        is_price_valid("10.1".to_string())
    );
    println!(
        "{}{:}",
        "Is the price valid +22.22.? ",
        is_price_valid("+22.22.".to_string())
    );
    println!(
        "{}{:}",
        "Is the price valid .100? ",
        is_price_valid(".100".to_string())
    );
}

// Time complexity = O(n)
// Space complexity = O(1)
