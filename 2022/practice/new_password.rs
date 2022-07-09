fn main() {

    let mut t = String::new();
    std::io::stdin().read_line(&mut t).expect("Error Reading");
    let test_cases:usize = t.trim().parse().expect("Error");

    for i in 0..test_cases {
        let mut useless = String::new();
        std::io::stdin().read_line(&mut useless).expect("Error");
        let mut old_password = String::new();
        std::io::stdin().read_line(&mut old_password).expect("Error reading");
        old_password.pop();
        let mut length = old_password.len();
        let mut new_password = old_password;

        let mut small_letters = false;
        let mut cap_letters = false;
        let mut numbers = false;
        let mut symbols = false;

        for i in 0..length {
            if new_password.as_bytes()[i] >= b'A' && new_password.as_bytes()[i]<= b'Z' {
                cap_letters = true;
            }
            if new_password.as_bytes()[i] >= b'a' && new_password.as_bytes()[i]<= b'z' {
                small_letters = true;
            }
            if new_password.as_bytes()[i] >= b'0' && new_password.as_bytes()[i]<= b'9' {
                numbers = true;
            }
            if new_password.contains('@') || new_password.contains('#') || new_password.contains('&') || new_password.contains('*'){
                symbols = true;
            }
        }

        if !small_letters {new_password.push('a')};
        if !cap_letters {new_password.push('A')};
        if !numbers {new_password.push('1')};
        if !symbols {new_password.push('@')};

        while new_password.len() < 7 {new_password.push('1')};
        println!("Case #{}: {}", i+1, new_password);
    }

}