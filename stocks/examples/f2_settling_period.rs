use std::cmp;

fn least_time(stocks: Vec<char>, s_time: i32) -> i32 {
    // frequencies of the stocks
    let mut frequencies: Vec<i32> = vec![0; 26];
    for s in stocks.clone().into_iter() {
        frequencies[(s as u32 - 'A' as u32) as usize] += 1;
    }
    frequencies.sort();

    // max frequency
    let f_max = frequencies[25];
    let mut idle_intervals = (f_max - 1) * s_time;
    let mut i: i32 = frequencies.len() as i32 - 2;
    while i >= 0 && idle_intervals > 0 {
        idle_intervals -= cmp::min(f_max - 1, frequencies[i as usize]);
        i -= 1;
    }

    if idle_intervals > 0 {
        return idle_intervals + stocks.len() as i32;
    } else {
        return stocks.len() as i32;
    }
}

fn main() {
    // Driver code
    let transaction: Vec<char> = vec!['A', 'A', 'A', 'T', 'T', 'M', 'A'];
    let k = 2;
    println!(
        "{}{:}{}",
        "Time requires to trade all stocks: ",
        least_time(transaction, k),
        " intervals"
    );
}

// Time complexity = O(n)
// Space complexity = O(1)
