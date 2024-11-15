use std::collections::HashMap;
use crate::http::next_token;

#[derive(Debug)]
pub struct QueryString<'request_buffer> {
    items: HashMap<&'request_buffer str, QueryParameterValue<'request_buffer>>,
}

impl <'request_buffer> QueryString<'request_buffer> {
    fn new() -> Self {
        let items = HashMap::new();
        Self { items }
    }

    fn add_value(&mut self, key: &'request_buffer str, value: &'request_buffer str) {
        self.items.entry(key).and_modify(|current| {
            match current {
                QueryParameterValue::Single(single) => {
                    let mut vec: Vec<&str> = Vec::new();
                    vec.push(single);
                    vec.push(value);
                    *current = QueryParameterValue::Multiple(vec)
                }
                QueryParameterValue::Multiple(current) => current.push(value)
            }
        }).or_insert(QueryParameterValue::Single(value));
    }
}

#[derive(Debug)]
pub enum QueryParameterValue<'request_buffer>{
    Single(&'request_buffer str),
    Multiple(Vec<&'request_buffer str>),
}

impl<'request_buffer> From<&'request_buffer str> for QueryString<'request_buffer> {
    fn from(value: &'request_buffer str) -> Self {
        let mut query_string = QueryString::new();

        for parameter in value.split('&') {
            match next_token(parameter, '=') {
                Some((key, value)) => query_string.add_value(key, value),
                None => query_string.add_value(parameter, "")
            }
        }

        query_string
    }
}