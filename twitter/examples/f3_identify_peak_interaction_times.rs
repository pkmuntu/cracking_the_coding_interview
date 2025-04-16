fn peak_interaction_times(interactions: Vec<i32>, hours: i32)-> Vec<i32> {
    let temp = interactions.len() as i32 - hours + 1;
    let mut sums: Vec<i32> = vec![0;temp as usize];
    let mut curr_sum: i32 = 0;
    for i in 0..interactions.len(){
        curr_sum += interactions[i];
        if i >= hours as usize {
            curr_sum -= interactions[i - hours as usize];
        }
        if i >= (hours - 1) as usize{
            sums[((i as i32- hours ) + 1) as usize ] = curr_sum;
        }
    }
    let size = sums.len();
    let mut left: Vec<i32> = vec![0;size as usize];
    let mut best = 0;
    for i in 0..sums.len() {
        if sums[i] > sums[best]
         {best = i;}
        left[i] = best as i32;
    }
    let mut right: Vec<i32> = vec![0;size as usize];
    best = sums.len() - 1;
    for i in (0..sums.len()).rev() {
        if sums[i] >= sums[best] {
            best = i;
        }
        right[i] = best as i32;
    }
    let mut output: Vec<i32> = vec![-1, -1, -1];
    for j in hours..(sums.len() as i32- hours) {
        let i = left[(j - hours) as usize];
        let l = right[(j + hours) as usize];
        if output[0] == -1 || sums[i as usize] + sums[j as usize] + sums[l as usize] > sums[output[0] as usize] + sums[output[1] as usize] + sums[output[2] as usize] {
            output[0] = i;
            output[1] = j;
            output[2] = l;
        }
    }
    return output;
}


fn main() {
    let interaction: Vec<i32> = vec![0, 2, 1, 3, 1, 7, 11, 5, 5];
    let hours = 2;
    let output: Vec<i32> = peak_interaction_times(interaction, hours);
    println!("{:?}",output);
}

// Time complexity = O(n)
// Space complexity = O(n)
