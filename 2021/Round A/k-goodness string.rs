use std::io;
fn main(){
    let mut t = String::new();
    io::stdin().read_line(&mut t).expect("Error");
    let test_cases:u8 = t.trim().parse().expect("Error");

    //each test_case
    for i in 1..test_cases+1 {
        //setup
        let mut inp = String::new();
        io::stdin().read_line(&mut inp).expect("Error");
        let a:Vec<&str> = inp.split_whitespace().collect();
        let mut length:usize = a[0].parse().unwrap();
        length -=1;
        let req_k:u32 = a[1].parse().unwrap();

        //input string
        let mut inp_string = String::new();
        io::stdin().read_line(&mut inp_string).expect("Error");
        let my_vec: Vec<char> = inp_string.chars().collect();

        

        //calculate goodness
        let mut score = 0;
        let mut ans = 0;
        for i in 0..length/2 {
            println!("checking {} and {}", my_vec[i], my_vec[length-i]);
            if my_vec[i] != my_vec[length-i] {
                score+=1;
            }
        }
        println!("Case #{}: {}", i, req_k-score);
    }
}

//let first_letter = &inp_string[1..2];
//println!("{}", first_letter);
