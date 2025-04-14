pub struct PeakUsers {
    // for the brute force solution
    pub users: Vec<Vec<i32>>,
    // for the caching rows solution
    pub cache: Vec<Vec<i32>>,
    pub smart_cache: Vec<Vec<i32>>,
}

impl PeakUsers {
    pub fn new(users: Vec<Vec<i32>>) -> Self {
        // Section Start - Brute Force
        PeakUsers {
            users: users.clone(),
            cache: vec![vec![]],
            smart_cache: vec![vec![]],
        };
        // Section Stop - Brute Force

        // Section Start - Caching Rows
        if users.len() == 0 || users[0].len() == 0 {
            let s: Vec<Vec<i32>> = Vec::new();
            return Self {
                users: users,
                cache: s.to_vec(),
                smart_cache: s.to_vec(),
            };
        }
        let mut cach: Vec<Vec<i32>> = vec![vec![0; users[0].len() + 1]; users.len()];
        for i in 0..users.len() {
            for j in 0..users[0].len() {
                cach[i][j + 1] = cach[i][j] + users[i][j];
            }
        }
        // Section End - Caching Rows

        // Section Start - Caching Smart
        if users.len() == 0 || users[0].len() == 0 {
            let s: Vec<Vec<i32>> = Vec::new();
            return Self {
                users: users,
                cache: s.to_vec(),
                smart_cache: s.to_vec(),
            };
        }
        let mut smartcach: Vec<Vec<i32>> = vec![vec![0; users[0].len() + 1]; users.len() + 1];

        for i in 0..users.len() {
            for j in 0..users[0].len() {
                smartcach[i + 1][j + 1] =
                    smartcach[i + 1][j] + smartcach[i][j + 1] + users[i][j] - smartcach[i][j];
            }
        }
        // Section End - Caching Smart
        PeakUsers {
            users: users,
            cache: cach,
            smart_cache: smartcach,
        }
    }

    // Brute Force
    pub fn brute_force(&mut self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let mut sum: i32 = 0;

        for r in row1..row2 + 1 {
            for c in col1..col2 + 1 {
                sum += self.users[r as usize][c as usize];
            }
        }
        return sum;
    }

    // Caching Rows
    pub fn cache_rows(&mut self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let mut sum: i32 = 0;
        for i in row1..row2 + 1 {
            sum +=
                self.cache[i as usize][col2 as usize + 1] - self.cache[i as usize][col1 as usize];
        }
        return sum;
    }

    // Caching Smarter
    pub fn cache_smart(&mut self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        return self.smart_cache[row2 as usize + 1][col2 as usize + 1]
            - self.smart_cache[row1 as usize][col2 as usize + 1]
            - self.smart_cache[row2 as usize + 1][col1 as usize]
            + self.smart_cache[row1 as usize][col1 as usize];
    }
}

fn main() {
    let users: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let mut pu = PeakUsers::new(users);
    //Example - 1
    let mut row1 = 0;
    let mut col1 = 1;
    let mut row2 = 2;
    let mut col2 = 2;
    println!("{}", "Example - 1:");
    println!(
        "{}{:}",
        "Peak Number of Users (Brute Force): ",
        pu.brute_force(row1, col1, row2, col2)
    );
    println!(
        "{}{:}",
        "Peak Number of Users (Caching Rows): ",
        pu.cache_rows(row1, col1, row2, col2)
    );
    println!(
        "{}{:}",
        "Peak Number of Users (Caching Smarter): ",
        pu.cache_smart(row1, col1, row2, col2)
    );

    // // Example - 2
    row1 = 1;
    col1 = 1;
    row2 = 2;
    col2 = 2;
    println!("{}", "Example - 2:");
    println!(
        "{}{:}",
        "Peak Number of Users (Brute Force): ",
        pu.brute_force(row1, col1, row2, col2)
    );
    println!(
        "{}{:}",
        "Peak Number of Users (Caching Rows): ",
        pu.cache_rows(row1, col1, row2, col2)
    );
    println!(
        "{}{:}",
        "Peak Number of Users (Caching Smarter): ",
        pu.cache_smart(row1, col1, row2, col2)
    );

    //Example - 3
    row1 = 0;
    col1 = 0;
    row2 = 2;
    col2 = 2;
    println!("{}", "Example - 3:");
    println!(
        "{}{:}",
        "Peak Number of Users (Brute Force): ",
        pu.brute_force(row1, col1, row2, col2)
    );
    println!(
        "{}{:}",
        "Peak Number of Users (Caching Rows): ",
        pu.cache_rows(row1, col1, row2, col2)
    );
    println!(
        "{}{:}",
        "Peak Number of Users (Caching Smarter): ",
        pu.cache_smart(row1, col1, row2, col2)
    );
}

//  Method        Time complexity         Space complexity
//  brute_force       O(m*n)                  O(1)
//  cache_rows        O(m)                    O(m * n)
//  cache_smart       O(1)                    o(m*n)
