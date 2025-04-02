fn find_servers(servers: Vec<i32>, k: i32, num: i32) -> Vec<i32> {
    let mut machines: Vec<i32> = Vec::new();

    // if the number of server is same as k, return the original server array
    if servers.len() == k as usize {
        for i in 0..k {
            machines.push(servers[i as usize]);
        }
        return machines;
    }

    // Do a binary search to find the element closest to num
    let mut left: i32 = 0;
    let mut right = servers.len() as i32;
    let mut mid: i32;
    while left < right {
        mid = (left + right) / 2;
        if servers[mid as usize] >= num {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    // Initialize the pointers for the sliding window
    left -= 1;
    right == left + 1;

    // while the sliding window's size is less than k
    while right - left - 1 < k {
        // check for out of bounds
        if left as i32 == -1 {
            right += 1;
            continue;
        }

        // Expand the window towards the side with the closer number
        // be careful to not go out of bounds with the pointer
        // |a - x| < |b -x|,
        // |a -x| == |b -x|
        if right == servers.len() as i32
            || (servers[left as usize] - num).abs() <= (servers[right as usize] - num).abs()
        {
            left -= 1;
        } else {
            right += 1;
        }
    }

    // Build and return the window
    for i in left + 1..right {
        machines.push(servers[i as usize]);
    }
    return machines;
}

fn main() {
    // Deriver code
    let servers: Vec<i32> = vec![1, 2, 3, 4, 5];
    let k = 4;
    let num = 3;
    let k_machines = find_servers(servers, k, num);
    println!("{:?}", k_machines);
}

// Time complexity = O(log(n) + k)
// Space complexity = o(1)
