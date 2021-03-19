use std::io;

fn main() {
    
    let mut t = String::new();
    io::stdin().read_line(&mut t).expect("Error");
    let test_cases: u8 = t.trim().parse().expect("Error");
    for i in 1..(test_cases+1) {
        //input
        let mut inp = String::new();
        io::stdin().read_line(&mut inp).expect("Error");
        let split = inp.split_whitespace();
        let vec = split.collect::<Vec<&str>>();
        let lower = vec[0].parse::<u64>().unwrap();
        let higher = vec[1].parse::<u64>().unwrap() + 1;

        //////////logic
        let mut boring = 0;
        let mut num_index = lower;

        while num_index < higher {
            //each number
            //setup
            let mut vec = Vec::new();
            let mut num = i;
            let mut is_boring = true;
            //parsing int to number
            while num != 0 {
                
                vec.insert(0, num%10);
                num /= 10;
            }
            
            // check if each is boring
            for i in 0..vec.len(){
                //println!("Checking if {} is boring.", vec[i]);
                if (i%2==0 && vec[i]%2!=0) || (i%2!=0 && vec[i]%2==0) {
                    
                } else {
                    is_boring = false;
                    break;
                }
            }
            if is_boring==true {
                boring+=1;
                //println!("{:?}", vec);
            }

            num_index+=1;
        }
        println!("Case #{}: {}", i, boring);
    }
}
