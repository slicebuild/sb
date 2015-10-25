use slice::item::Slice;

pub fn code_for_slice(slice: &Slice) -> String {
    let mut string = String::new();
    for item in slice.run_section() {
        string.push_str(item);
        string.push('\n');
    }
    string
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use version;
    use slice::item::Slice;
    use slice::section::{Kind, Section};

    fn create_slice(run_section_items: Vec<&str>) -> Slice {
        let items = run_section_items.into_iter().map(|i| i.to_string()).collect::<Vec<_>>();
        let run_section = Section { kind: Kind::Run, items: items };
        Slice { name: "Hello".to_string(), path: PathBuf::new(),
                version: version::zero(), sections: vec![run_section] }
    }

    #[test]
    fn code_for_slice() {
        let slice = create_slice(vec!["apt-get install -q -y wget"]);
        assert_eq!(super::code_for_slice(&slice), "apt-get install -q -y wget\n");

        let run_section_items = vec!["apt-get install -q -y wget",
                                     "apt-get install -q -y wget_gui"];
        let slice = create_slice(run_section_items);
        assert_eq!(super::code_for_slice(&slice), "apt-get install -q -y wget
apt-get install -q -y wget_gui
");
    }
}