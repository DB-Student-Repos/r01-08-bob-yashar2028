pub fn reply(message: &str) -> &str {
    let message = message.trim();
    let question = message.ends_with("?");
    let upper = message.contains(char::is_alphabetic) && message == message.to_uppercase();
    if message.is_empty() {
          "Fine. Be that way!"     
    } else if question && upper {
          "Calm down, I know what I'm doing!" 
    } else if question {
          "Sure."
    } else if upper {
          "Whoa, chill out!"
    } else {
          "Whatever."
    }
}
