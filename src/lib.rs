pub fn reply(message: &str) -> &str {
    let mut response: &str = "";

    if message.trim().ends_with("?") {
        if message == message.trim().to_uppercase() && is_alphabetic(message) {
            response = "Calm down, I know what I'm doing!";
        } else {
            response = "Sure."
        }
    } else if message == message.trim().to_uppercase()
        && message.trim() != ""
        && is_alphabetic(message)
    {
        response = "Whoa, chill out!";
    } else if message.trim() == "" {
        response = "Fine. Be that way!";
    } else {
        response = "Whatever.";
    }

    response
}

fn is_alphabetic(s: &str) -> bool {
    let mut has_alphabet = false;
    s.chars().for_each(|x| {
        if x.is_alphabetic() {
            has_alphabet = true
        }
    });
    has_alphabet
}
