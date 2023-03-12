pub fn verse(n: u32) -> String {
    let s1 = String::from(
        if n > 0 {
            n.to_string()
        }
        else {
            "no more".to_string()
        }
    );
}

pub fn sing(start: u32, end: u32) -> String {
    unimplemented!("sing verses {start} to {end}, inclusive")
}
