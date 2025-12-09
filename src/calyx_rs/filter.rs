pub trait Filter {
    fn apply(&self, string: &mut String);
}
