use slice::item::Slice;

pub fn code_for_slice(slice: &Slice) -> String {
    let mut string = String::new();
    for item in slice.run_section() {
        string.push_str(item);
        string.push('\n');
    }
    string
}