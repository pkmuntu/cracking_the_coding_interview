use std::cmp;
fn update_global_profile(x_coordinate: i32, y_coordinate: i32,  global_profile:&mut Vec<Vec<i32>>)-> Vec<Vec<i32>>{
    // Updating and returning our global profile 
    // add a new point if our global profile has not changed vertically.
    if global_profile.len() == 0 || global_profile[global_profile.len() - 1][0] != x_coordinate
       { global_profile.push(vec![x_coordinate, y_coordinate]);}
    else {
    // update the last point if our global profile has changed vertically
        let l = global_profile.clone().len();
        global_profile[l - 1][1] = y_coordinate;
      }

    return global_profile.to_vec();
}

fn appendglobal_profile(curr_index:&mut i32,  tweet_list: Vec<Vec<i32>>, tweet_list_len: i32, c_mentions:&mut i32,  global_profile:&mut Vec<Vec<i32>>){
    // From our current index, append the rest of the elements to our final global profile.
    while *curr_index < tweet_list_len{
        let x_coordinate = tweet_list[*curr_index as usize][0];
        let y_coordinate = tweet_list[*curr_index as usize][1];
        *curr_index += 1;
        if *c_mentions != y_coordinate {
            *global_profile = update_global_profile(x_coordinate, y_coordinate,&mut global_profile.to_vec());
            *c_mentions = y_coordinate;
        }
    }
}

fn mergeglobal_profile( start_tweets: Vec<Vec<i32>>, end_tweets: Vec<Vec<i32>>)-> Vec<Vec<i32>>{
    // Storing the size()s of both global profiles i.e., starting tweets and ending tweets.
    let len_start = start_tweets.len();
    let len_end = end_tweets.len();

    // initializing the variables.
    let mut i_start = 0;
    let mut i_end = 0;
    let mut c_mentions = 0;
    let mut start_y = 0;
    let mut end_y = 0;
    let mut x_coordinate;
    let mut global_profile: Vec<Vec<i32>> = Vec::new();

    // while we're in the region where both global_profiles are present
    while i_start < len_start && i_end < len_end{
        let start_point = start_tweets[i_start].to_vec();
        let end_point = end_tweets[i_end].to_vec();

        // pick up the smallest x
        if start_point[0] < end_point[0]{
            x_coordinate = start_point[0];
            start_y = start_point[1];
            i_start += 1;
        }
        else{
            x_coordinate = end_point[0];
            end_y = end_point[1];
            i_end += 1;
        }
        // max height (i.e. y) between both global_profiles
        let max_peak_mentions = cmp::max(start_y, end_y);

        // if there is a global_profile change
        if c_mentions != max_peak_mentions{
            global_profile = update_global_profile(x_coordinate, max_peak_mentions,&mut global_profile);
            c_mentions = max_peak_mentions;
        }
    }
    // // there is only start_tweets global_profile
    appendglobal_profile(&mut (i_start as i32), start_tweets, len_start as i32,&mut c_mentions,&mut global_profile);

    // // there is only end_tweets global_profile
    appendglobal_profile(&mut (i_end as i32), end_tweets, len_end as i32,&mut c_mentions,&mut global_profile);

    return global_profile;
}

fn findglobal_profile( tweets:&mut Vec<Vec<i32>>)-> Vec<Vec<i32>>{
    // Finding the lenght of the tweets list
    let tweet_len = tweets.len();

    // If the tweets list is empty, return an empty list.
    if tweet_len == 0
    {    return vec![];}

    // If there is only tweet in the list, return the required coordinates 
    if tweet_len == 1{
        let startx_coordinate = tweets[0][0];
        let endx_coordinate  = tweets[0][1];
        let peak_mentions = tweets[0][2];
        return vec![vec![startx_coordinate, peak_mentions], vec![endx_coordinate, 0]];
    }

    // If there is more than one tweet, keep dividing the tweets recurssively into two parts.
    let f = (tweet_len as f32/2.0).floor();
    
    let mut end: Vec<Vec<i32>> = tweets.split_off(f as usize);
    let mut start: Vec<Vec<i32>> = tweets.to_vec();
    let start_tweets = findglobal_profile(&mut start);
    let end_tweets = findglobal_profile(&mut end);

    // // Merge the divided tweets together.
    return mergeglobal_profile(start_tweets, end_tweets);
}




fn main() {
  // your code goes here
  let mut tweets: Vec<Vec<i32>> = vec![vec![2,9,10], vec![3,7,15], vec![5,12,12], vec![15,20,10], vec![19,24,8]];
  let result = findglobal_profile(&mut tweets);
  println!("{:?}",result);
}

// Time complexity = O(nlogn)
// Space complexity = o(n)
