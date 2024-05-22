

pub mod counter {
    pub fn message_counter(user_handle: &str, content: &str) -> u32 {
        println!("searching for ...{}", user_handle);

        //splitting the string into lines 
        let lines: Vec<&str> = content.lines().collect();

        let mut count = 0;

        for item in lines{
            println!("Checking: {}", item);
            if item.contains(user_handle){
                println!("Match found: {}", item);
                count += 1;
            }
        }
        
        println!("Total matches: {}", count);

        count
    }
}

