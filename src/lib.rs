pub mod string {
    pub fn replace(base: &str, target: &str, src: &str) -> String {
        let mut new: String = String::new();
    
        let mut target_chars = target.chars();
    
        let mut start_indices: Vec<u32> = Vec::<u32>::new();
        let mut current_start_index: Option<u32> = None;
    
        let mut index: u32 = 0;
    
        for c in base.chars() {
            let target_char: Option<char> = target_chars.next();
    
            match target_char {
                None => {
                    if current_start_index != None {
                        start_indices.push(match current_start_index {
                            None => 0,
                            Some(num) => num,
                        });
                        target_chars = target.chars();
                    }
                },
                Some(t_char) => {
                    if c == t_char && current_start_index == None {
                        current_start_index = Some(index);
                    } else if c != t_char {
                        current_start_index = None;
                        target_chars = target.chars();
                    }
                },
            };
            index += 1;
        }
    
        if current_start_index != None {
            start_indices.push(match current_start_index {
                None => 0,
                Some(num) => num,
            });
        }
    
        let mut is_replacing: bool = false;
        target_chars = target.chars();
    
        index = 0;
        for c in base.chars() {
            if start_indices.contains(&index) {
                is_replacing = true;
    
                for l in src.chars() {
                    new.push(l);
                }
            }
    
            if is_replacing {
                if let None = target_chars.next() {
                    is_replacing = false;
                    new.push(c);
                };
            } else {
                new.push(c);
            }
    
            index += 1;
        }
    
        new
    }
}