use slice::Slice;

pub fn generate_code(slice: &Slice) -> String {
    let mut string = String::new();
    if ! slice.ancestors().is_empty() {
        for ancestor in slice.ancestors() {
            string.push_str(&format!("FROM {}\n", ancestor));
        }
        string.push('\n');
    }

    string.push_str("RUN ");
    let mut is_first = true;
    for preparation in slice.preparations() {
        if is_first {
            is_first = false;
        } else {
            string.push_str(" && \\\n");
        }
        string.push_str(preparation);
    }
    string.push('\n');
    string
}

#[cfg(test)]
mod tests {
    use slice::Slice;
    use version;

    fn create_slice(ancestors: Vec<&str>, preparations: Vec<&str>) -> Slice {
        let ancestors = ancestors.into_iter().map(str::to_string).collect();
        let preparations = preparations.into_iter().map(str::to_string).collect();
        let name = String::from("Hello");
        Slice::new(name, version::zero(), ancestors, preparations, Vec::new(), Vec::new())
    }

    #[test]
    fn generate_code_for_slice_with_one_preparation() {
        let slice = create_slice(Vec::new(), vec!["apt-get install -q -y wget"]);
        assert_eq!(super::generate_code(&slice), "RUN apt-get install -q -y wget\n");
    }

    #[test]
    fn generate_code_for_slice_with_one_ancestor_and_one_preparation() {
        let slice = create_slice(vec!["base"], vec!["apt-get install -q -y wget"]);
        assert_eq!(super::generate_code(&slice), "FROM base

RUN apt-get install -q -y wget\n");
    }

    #[test]
    fn generate_code_for_slice_with_two_ancestors_and_two_preparations() {
        let slice = create_slice(vec!["base", "another_base"],
                                 vec!["apt-get install -q -y wget",
                                      "apt-get install -q -y wget_gui"]);
        assert_eq!(super::generate_code(&slice), "FROM base
FROM another_base

RUN apt-get install -q -y wget && \\
apt-get install -q -y wget_gui
");
    }
}
