use slice::Slice;

pub fn generate_code(slice: &Slice) -> String {
    let mut string = String::new();
    for preparation in slice.preparations() {
        string.push_str(preparation);
        string.push('\n');
    }
    string
}

#[cfg(test)]
mod tests {
    use slice::Slice;
    use version;

    fn create_slice(ancestors: Vec<&str>, preparations: Vec<&str>) -> Slice {
        let ancestors = ancestors.into_iter().map(str::to_string).collect();
        let preparations = preparations.into_iter().map(str::to_string).collect();
        let name = String::from("slice");
        Slice::new(name, version::zero(), ancestors, preparations, Vec::new(), Vec::new())
    }

    #[test]
    fn generate_code_for_slice_with_one_ancestor_and_one_preparation() {
        let slice = create_slice(vec!["base"], vec!["apt-get install -q -y wget"]);
        assert_eq!(super::generate_code(&slice), "apt-get install -q -y wget\n");
    }

    #[test]
    fn generate_code_for_slice_with_one_ancestor_and_two_preparations() {
        let slice = create_slice(vec!["base"], vec!["apt-get install -q -y wget",
                                                    "apt-get install -q -y wget_gui"]);
        assert_eq!(super::generate_code(&slice), "apt-get install -q -y wget
apt-get install -q -y wget_gui
");
    }
}