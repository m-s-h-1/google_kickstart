fn main(){
    let mut t = String::new();
    std::io::stdin().read_line(&mut t).expect("Error");
    let test_cases:u8 = t.trim().parse().expect("Error");

    //each test_case
    for i in 1..test_cases+1 {
        //setup
        let mut inp = String::new();
        std::io::stdin().read_line(&mut inp).expect("Error");
        let a:Vec<&str> = inp.split_whitespace().collect();
        let n:usize = a[0].parse().unwrap();
        let req_k:i32 = a[1].parse().unwrap();

        //input string
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).expect("Error");

        let mut cur_score = 0;
        for i in 0..(n/2) {
            if s.as_bytes()[i] != s.as_bytes()[n-1-i] {
                cur_score += 1;
            }
        }

        println!("Case #{}: {}", i, ((cur_score-req_k) as i32).abs());
    }
}