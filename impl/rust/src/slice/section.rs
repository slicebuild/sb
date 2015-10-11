#[derive(Debug)]
#[derive(PartialEq)]
pub enum Kind {
    Os,
    Dep,
    Add,
    Cmd,
    Copy,
    EntryPoint,
    Env,
    Expose,
    From,
    Label,
    Maintainer,
    Onbuild,
    Run,
    User,
    Volume,
    WorkDir
}

impl Kind {
    fn from_string(string: &String) -> Option<Kind> {
        let string: &str = string;
        match string {
            "OS" => Some(Kind::Os),
            "DEP" => Some(Kind::Dep),
            "ADD" => Some(Kind::Add),
            "CMD" => Some(Kind::Cmd),
            "COPY" => Some(Kind::Copy),
            "ENTRYPOINT" => Some(Kind::EntryPoint),
            "ENV" => Some(Kind::Env),
            "EXPOSE" => Some(Kind::Expose),
            "FROM" => Some(Kind::From),
            "LABEL" => Some(Kind::Label),
            "MAINTAINER" => Some(Kind::Maintainer),
            "ONBUILD" => Some(Kind::Onbuild),
            "RUN" => Some(Kind::Run),
            "USER" => Some(Kind::User),
            "VOLUME" => Some(Kind::Volume),
            "WORKDIR" => Some(Kind::WorkDir),
            _ => None
        }
    }
}

#[derive(Debug)]
pub struct Section {
    pub kind: Kind,
    pub items: Vec<String>
}

impl Section {
    pub fn load_from_lines(lines: Vec<String>) -> (Option<Section>, Vec<String>) {
        let mut remaining_lines: Vec<String> = Vec::new();
        let mut items: Vec<String> = Vec::new();
        let mut section_type: Option<Kind> = None;
        let mut section_end_reached = false;
        for line in lines {
            if section_end_reached {
                remaining_lines.push(line);
                continue;
            }
            if line.is_empty() {
                continue;
            }
            if let Some(section_type_from_line) = Kind::from_string(&line) {
                if let Some(_) = section_type {
                    section_end_reached = true;
                    remaining_lines.push(line);
                } else {
                    section_type = Some(section_type_from_line);
                }
            } else {
                if let Some(_) = section_type {
                    items.push(line);
                }
            }
        }
        if let Some(section_type) = section_type {
            let section = Section { kind: section_type, items: items };
            (Some(section), remaining_lines)
        } else {
            (None, remaining_lines)
        }
    }
}

#[test]
fn section_load_from_lines_works() {
    let mut lines: Vec<String> = Vec::new();
    lines.push("OS".to_string());
    lines.push("debian-8.2".to_string());
    lines.push("".to_string());
    lines.push("FROM".to_string());
    lines.push("debian:jessie".to_string());
    let (section, lines) = Section::load_from_lines(lines);
    let section = section.unwrap();
    assert_eq!(section.kind, Kind::Os);
    assert_eq!(section.items.len(), 1);
    assert_eq!(section.items.first().unwrap(), "debian-8.2");
    let (section, lines) = Section::load_from_lines(lines);
    let section = section.unwrap();
    assert_eq!(section.kind, Kind::From);
    assert_eq!(section.items.len(), 1);
    assert_eq!(section.items.first().unwrap(), "debian:jessie");
}
