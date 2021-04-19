fn main() {
    let mut t = String::new();
    std::io::stdin().read_line(&mut t).expect("Error");
    let test_cases:usize = t.trim().parse().expect("Error");

    for index in 1..test_cases+1 {
        let mut num = String::new();
        std::io::stdin().read_line(&mut num).expect("Error");
        let length:usize = num.trim().parse().unwrap();

        let mut inp = String::new();
        std::io::stdin().read_line(&mut inp).expect("Error");
        let mut last_char = 'Z';
        print!("Case #{}: ", index);
        let mut v = 0;
        for i in 0..length {
            let a = inp.as_bytes()[i];
            let c = a as char;

            //println!("c:{} and last:{}", c, last_char);

            if c > last_char {
                v+=1;
            } else {
                v=1;
            }
            last_char = c;
            print!("{} ", v);
        }
        println!("");
    }
}