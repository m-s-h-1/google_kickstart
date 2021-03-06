use std::io;
use std::cmp;
use cmp::min;

fn main(){
    let mut t = String::new();
    io::stdin().read_line(&mut t).expect("Error");
    let test_cases:u8 = t.trim().parse().expect("Some error");
    let mut i = 1;
    while i <= test_cases {
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Something!");
        let mut split = input.split_whitespace();
        let mut vec = split.collect::<Vec<&str>>();
        let mut N = vec[0].parse::<i32>().unwrap();
        let mut K = vec[1].parse::<i32>().unwrap();
        let mut S = vec[2].parse::<i32>().unwrap();
        let mut ans = min((N+1), (K-S) + (N-S) + 1) + (K - 1);
        println!("Case #{}: {}", i, ans);

        i +=1;
    }
}