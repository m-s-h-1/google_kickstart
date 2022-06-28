fn main() {
    let mut t = String::new();
    std::io::stdin().read_line(&mut t).expect("Error");
    let test_cases:usize = t.trim().parse().expect("Error");

    for i in 1..test_cases+1 {
        let mut inp = String::new();
        std::io::stdin().read_line(&mut inp).expect("Error");
        let mut length = inp.len();
        inp.pop();
        let last_letter = inp[length-2..length-1].to_ascii_lowercase();

        print!("Case #{}: {} is ruled by ", i, inp);
        if last_letter == "a" || last_letter == "e" || last_letter == "i" || last_letter == "o" || last_letter == "u" {
            println!("Alice.");
        } else if last_letter == "y" {
            println!("nobody.")
        } else {
            println!("Bob.")
        }
    }
    
}