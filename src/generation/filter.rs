use std::collections::HashMap;

pub type Filter = fn(&mut String);

pub fn create_builtin_filters() -> HashMap<String, Filter> {
    let mut filters: HashMap<String, Filter> = HashMap::new();

    filters.insert("lowercase".to_string(), |s| s.make_ascii_lowercase());
    filters.insert("uppercase".to_string(), |s| s.make_ascii_uppercase());

    filters
}
