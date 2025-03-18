fn divide_posts(days: Vec<i32>, k: i32) -> i32 {
    let mut left = 1;
    let mut right = 0;

    for i in 0..days.len() {
        right = right + days[i];
    }
    right = right / k;

    while left < right {
        // half to the data
        let mid = (left + right + 1) / 2;

        // This would denote the posts we currently have as we are traversing over
        // the list
        let mut target = 0;

        // This would tell us how many days we would get after dividing
        // the list in `mid` amount of posts
        let mut divisions = 0;
        for posts in &days {
            target += posts;
            if target >= mid {
                divisions += 1;
                target = 0;
            }
        }
        if divisions >= k {
            left = mid;
        } else {
            right = mid - 1;
        }
    }
    return left;
}

fn main() {
    // Driver code

    let days = vec![1000, 2000, 3000, 4000, 5000];
    let nodes = 4;
    println!(
        "{}{}{}",
        "The master node was assigned ",
        divide_posts(days, nodes).to_string(),
        " posts"
    );
}

// Time complexity = O(n * log(m))
// Space complexity = O(1)
