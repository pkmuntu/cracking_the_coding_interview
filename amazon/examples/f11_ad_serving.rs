// BufferState maintains the state of buff that is passed to read4()
pub struct BufferState {
    buff: Vec<char>,
    ptr: i32,
    count: i32,
}

impl BufferState {
    pub fn new(ptr: i32, count: i32) -> Self {
        BufferState {
            ptr: ptr,
            count: count,
            buff: vec![],
        }
    }
}

fn read(buffer: &mut Vec<char>, n: i32, state: &mut BufferState) -> i32 {
    let mut i = 0;

    while i < n {
        // Reading the ads list till the given n
        // If ptr is 0 call the read4() API and read 4 ads from the list of ads
        if state.ptr == 0 {
            state.count = read4(&mut state.buff);
        }
        // Break the loop if list of ads is empty or read4() returns 0
        if state.count == 0 {
            break;
        }
        // Fill the buffer array using the already filled buff array
        // till the n ads.
        while i < n && state.ptr < state.count {
            // Append the ad to buffer
            buffer.push(state.buff[state.ptr as usize]);
            i += 1;
            state.ptr += 1;
        }
        // If ptr exceeds or reaches the count, the ptr will be set to 0
        if state.ptr >= state.count {
            state.ptr = 0;
        }
    }
    return i; // Return the number of ads read
}

fn main() {
    // driver code

    let mut count;
    let mut buffer: Vec<char> = Vec::new();
    let mut state: BufferState = BufferState::new(0, 0);
    // List of ads
    println!("{}{}", "Ads:", ADS);

    let queries: Vec<i32> = vec![3, 5, 2];

    //Example 1
    println!("Example #1");
    println!("{}{:}{}", "User: Read ", queries[0], " ads.");
    println!("{}{:}{}", "...reading ", queries[0], " ads...");
    count = read(&mut buffer, queries[0], &mut state);
    println!("{}{:}", "Ads count read by read(n): ", count);
    println!("{}{:?}", "Ads read by read(n): ", buffer);

    //Example 2
    buffer = vec![];
    println!("Example #2");
    println!("{}{:}{}", "User: Read ", queries[1], " ads.");
    println!("{}{:}{}", "...reading ", queries[1], " ads...");
    count = read(&mut buffer, queries[1], &mut state);
    println!("{}{:}", "Ads count read by read(n): ", count);
    println!("{}{:?}", "Ads read by read(n): ", buffer);

    // //Example 3

    buffer = vec![];
    println!("Example #3");
    println!("{}{:}{}", "User: Read ", queries[2], " ads.");
    println!("{}{:}{}", "...reading ", queries[2], " ads...");
    count = read(&mut buffer, queries[2], &mut state);
    println!("{}{:}", "Ads count read by read(n): ", count);
    println!("{}{:?}", "Ads read by read(n): ", buffer);
}

// Time complexity = O(n)
// Space complexity = O(1)
