pub mod counter {
    pub fn message_counter(user_handle: &str, content: &str) -> u32 {
        // when you type the username and press Enter,
        // this creates a newline at the end of the string,
        // `trim` removes newlines of the string type.
        //
        // before trim: "robertedwardgrant137\n"
        // after trim: "robertedwardgrant137"
        let user_handle = user_handle.trim();

        println!("searching for: {user_handle:#?}");

        println!("Checking: {}", content);

        let mut count = 0;

        // the variable `content` only has one line,
        // so we don't need to iter using `.lines()`
        if content.contains(user_handle) {
            println!("Match found");
            count += 1;
        }

        println!("Total matches: {}", count);

        count
    }
}
