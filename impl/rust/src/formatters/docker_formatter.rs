use slice::item::Slice;
use slice::section::Kind;

pub fn code_for_slice(slice: &Slice) -> String {
    let mut string = String::new();
    for item in slice.section(Kind::From) {
        string.push_str("FROM ");
        string.push_str(item);
        string.push('\n');
    }
    string.push_str("RUN ");
    let mut is_first = true;
    for item in slice.run_section() {
        if is_first {
            is_first = false;
        } else {
            string.push_str(" && \\\n");
        }
        string.push_str(item);
    }
    string
}