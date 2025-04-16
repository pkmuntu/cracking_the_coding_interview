fn add_likes(like1: String, like2: String) ->String{
    let mut res = String::from("");

    let mut carry = 0;
    let mut p1 = like1.len() as i32 - 1;
    let mut p2 = like2.len() as i32 - 1;
    while p1 >= 0 || p2 >= 0 {
        let x1;
        let x2;
        if p1 >= 0
        {
          x1 = like1.chars().nth(p1 as usize).unwrap()as u32 - '0' as u32;
        }
        else 
            {x1 = 0;}
        if p2 >= 0
        {
            x2 = like2.chars().nth(p2 as usize).unwrap() as u32 - '0' as u32;
        }
        else 
            {x2 = 0;}
        let value = (x1 + x2 + carry) % 10;
        carry = (x1 + x2 + carry) / 10;
        res += &value.to_string();
        p1-=1;
        p2-=1;    
    }
    
    if carry != 0
    {
      res += &carry.to_string();
    }
    return res.chars().rev().collect::<String>();
}
fn main() {
    println!("{}",add_likes("1545".to_string(), "67".to_string()));
}
// Time complexity = O(max(n1, n2))
// Space complexity = O(max(n1, n2))
