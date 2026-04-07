pub fn reply(message: &str) -> &str {
    let yelling = message.to_uppercase() == message && message.to_lowercase() != message;
    let question = message.trim().ends_with("?");

    if message.trim().is_empty() {
        return "Fine. Be that way!";
    }
    match (yelling, question) {
        (true, true) => "Calm down, I know what I'm doing!",
        (false, true) => "Sure.",
        (true, false) => "Whoa, chill out!",
        (_, _) => "Whatever.",
    }
}
