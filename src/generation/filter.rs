use std::collections::HashMap;

pub type Filter = fn(String) -> String;

pub(crate) fn create_builtin_filters() -> HashMap<String, Filter> {
    let mut filters: HashMap<String, Filter> = HashMap::new();

    filters.insert("lowercase".to_string(), |s| s.to_lowercase());
    filters.insert("uppercase".to_string(), |s| s.to_uppercase());
    filters.insert("length".to_string(), |s| format!("{}", s.len()));

    filters
}
