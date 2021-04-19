use std::io;
use aho_corasick::AhoCorasick;

fn main(){
    let mut t = String::new();
    io::stdin().read_line(&mut t).expect("Error");
    let test_cases:u8 = t.trim().parse().expect("Some error");
    
    for i in 1..test_cases+1 {
        let mut test_string = String::new();
        io::stdin().read_line(&mut test_string).expect("Failed");

        let patterns = &["KICK"];
        let mut haystack = &test_string;

        let ac = AhoCorasick::new(patterns);
        let mut matches = vec![];
        for mat in ac.find_iter(haystack) {
            matches.push((mat.pattern(), mat.start(), mat.end()));
        }
        println!("{:?}", matches);
        //let index = test_string.find("KICK").unwrap();
        //println!("{}", index);
    }
}