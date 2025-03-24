fn search(milestones: Vec<i32>, n: i32) -> i32 {
    let mut first: i32 = 0;
    let mut last: i32 = milestones.len() as i32;
    while first < last {
        let mid: i32 = (first + last) / 2;
        if milestones[mid as usize] >= n {
            last = mid;
        } else {
            first = mid + 1;
        }
    }
    return first;
}

fn milestone_days(milestones: &Vec<i32>, target: &i32) -> Vec<i32> {
    let first_day = search(milestones.to_vec(), *target);
    if *target == milestones[first_day as usize] {
        let last_day = search(milestones.to_vec(), *target + 1) - 1;
        return vec![first_day, last_day];
    } else {
        return vec![-1, -1];
    }
}

fn main() {
    let milestones: Vec<i32> = vec![0, 1, 1, 2, 2, 2, 3, 4, 4, 4, 5, 5, 6, 7];
    let target = 4;
    let res = milestone_days(&milestones, &target);
    println!("{:?}", res);
}

// Time complexity = O(log(n))
// Space complexity = O(1)
