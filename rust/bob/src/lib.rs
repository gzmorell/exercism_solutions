pub fn reply(message: &str) -> &str {
    if message.trim() == "" {
        "Fine. Be that way!"
    } else if message == message.to_uppercase() && message.contains(|x:char| x.is_alphabetic()){
        if message.trim().ends_with("?") {
            "Calm down, I know what I'm doing!"
        } else {
            "Whoa, chill out!"
        }
    }  else if message.trim().ends_with("?") {
        "Sure."
    } else {
        "Whatever."
    }
}
