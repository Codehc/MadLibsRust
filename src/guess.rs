pub fn get_num -> i32 {
    loop {
        let mut string = String::new();
        io::stdin().read_line(&mut string).expect("Please enter a valid string.");

        let parsed: i32 = match string.trim().parse() {
            Ok(result) => result,
            Err(_) => continue,
        };

        return parsed;
    }
}