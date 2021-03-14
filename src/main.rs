use std::io;

fn main() {
    let mut is_boring = true;
    let mut boring = 0;
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

        //////////logic

        for i in lower..higher {
            //each number
            //setup
            let mut vec = Vec::new();
            let mut num = i;
            //parsing int to number
            while num != 0 {
                
                vec.insert(0, num%10);
                num /= 10;
            }
            // check if each is boring
            for i in 0..vec.len(){
                if (i%2==0 && vec[i]%2!=0) || (i%2!=0 && vec[i]%2==0) {
                    is_boring = false;
                } else {
                    
                }
            }
            if is_boring==true {
                boring+=1;
                println!("{:?}", vec);
            }
        }
        println!("{}", boring);
    }
}
