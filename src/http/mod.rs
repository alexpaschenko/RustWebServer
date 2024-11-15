mod query_string;
pub mod request;
pub mod response;

fn next_token(request_str: &str, sep: char) -> Option<(&str, &str)> {
    for (i, ch) in request_str.chars().enumerate() {
        if ch == sep {
            return Some((&request_str[..i], &request_str[i + 1..]));
        }
    }

    None
}