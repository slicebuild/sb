use slice::item::Slice;
use slice::section::Kind;

pub fn code_for_slice(slice: &Slice) -> String {
    let mut string = String::new();
    for item in slice.section(Kind::From) {
        string.push_str("FROM ");
        string.push_str(item);
        string.push('\n');
    }
    let run_section = slice.run_section();
    if !run_section.is_empty() {
        string.push_str("RUN ");
        let mut is_first = true;
        for item in run_section {
            if is_first {
                is_first = false;
            } else {
                string.push_str(" && \\\n");
            }
            string.push_str(item);
        }
        string.push('\n');
    }
    string
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use helper;
    use slice::item::Slice;
    use slice::section::{Kind, Section};

    fn create_slice(from_section_items: Vec<&str>, run_section_items: Vec<&str>) -> Slice {
        let items = from_section_items.into_iter().map(|i| i.to_string()).collect::<Vec<_>>();
        let from_section = Section { kind: Kind::From, items: items };
        let items = run_section_items.into_iter().map(|i| i.to_string()).collect::<Vec<_>>();
        let run_section = Section { kind: Kind::Run, items: items };
        Slice { name: "Hello".to_string(), path: PathBuf::new(),
                version: helper::zero_version(), sections: vec![from_section, run_section] }
    }

    #[test]
    fn code_for_slice() {
        let slice = create_slice(Vec::new(), vec!["apt-get install -q -y wget"]);
        assert_eq!(super::code_for_slice(&slice), "RUN apt-get install -q -y wget\n");

        let slice = create_slice(vec!["base"], vec!["apt-get install -q -y wget"]);
        assert_eq!(super::code_for_slice(&slice), "FROM base
RUN apt-get install -q -y wget\n");

        let run_section_items = vec!["apt-get install -q -y wget",
                                     "apt-get install -q -y wget_gui"];
        let slice = create_slice(vec!["base", "another_base"], run_section_items);
        assert_eq!(super::code_for_slice(&slice), "FROM base
FROM another_base
RUN apt-get install -q -y wget && \\
apt-get install -q -y wget_gui
");
    }
}
