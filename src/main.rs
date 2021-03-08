use std::io;

fn main() {
    let mut t = String::new();
    io::stdin().read_line(&mut t).expect("Error");
    let test_cases: u8 = t.trim().parse().expect("Error");
    for _i in 1..(test_cases+1) {
        //input
        let mut inp = String::new();
        io::stdin().read_line(&mut inp).expect("Error");
        let split = inp.split_whitespace();
        let vec = split.collect::<Vec<&str>>();
        let lower = vec[0].parse::<u64>().unwrap();
        let higher = vec[1].parse::<u64>().unwrap() + 1;

        //logic

        //all numbers
        for i in lower..higher {
            //each number
            let mut num = i;
            let mut vec = Vec::new();
            while num != 0 {
                vec.push(num/10);
                num /= 10;
            }
            //check if digit is boring
            for i in 0..(vec.len()) {
                println!("Digit: {} and index: {}", vec[i], i);
                if (i%2==0 && vec[i]%2!=0) || (i%2!=0 && vec[i]==0) {
                    println!("boring");
                } else {
                    break;
                }
            }
        }
    }
}
