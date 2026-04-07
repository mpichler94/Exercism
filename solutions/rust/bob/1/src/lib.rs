pub fn reply(message: &str) -> &str {
    let yelling = message.to_uppercase() == message && message.to_lowercase() != message;
    let question = message.trim().ends_with("?");

    if message.trim().is_empty() {
        return "Fine. Be that way!";
    }

    if question {
        if yelling {
            return "Calm down, I know what I'm doing!";
        }
        return "Sure.";
    }

    if yelling {
        return "Whoa, chill out!";
    }

    "Whatever."
}
