fn main() {
    // this code cause ownership error
    // 
    // s1 have ownership
    // let s1 = String::from("hello");
    // 
    // move ownership from s1 to s2.
    // let s2 = s1;
    // 
    // occurred error, because s1 have not ownership.
    // println!("{s1}, world");
    // 
    
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    
    println!("The length of '{s1} is {len}'");
    
    println!("{}", s1[2..3].to_string());
    
    let bytes = first_word(&s1);
    println!("{bytes}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}