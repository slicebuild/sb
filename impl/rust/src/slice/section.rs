use std::borrow::Borrow;
use std::str::FromStr;

#[derive(Clone)]
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
    WorkDir,
}

impl FromStr for Kind {
    type Err = String;

    fn from_str(string: &str) -> Result<Kind, Self::Err> {
        match string {
            "OS" => Ok(Kind::Os),
            "DEP" => Ok(Kind::Dep),
            "ADD" => Ok(Kind::Add),
            "CMD" => Ok(Kind::Cmd),
            "COPY" => Ok(Kind::Copy),
            "ENTRYPOINT" => Ok(Kind::EntryPoint),
            "ENV" => Ok(Kind::Env),
            "EXPOSE" => Ok(Kind::Expose),
            "FROM" => Ok(Kind::From),
            "LABEL" => Ok(Kind::Label),
            "MAINTAINER" => Ok(Kind::Maintainer),
            "ONBUILD" => Ok(Kind::Onbuild),
            "RUN" => Ok(Kind::Run),
            "USER" => Ok(Kind::User),
            "VOLUME" => Ok(Kind::Volume),
            "WORKDIR" => Ok(Kind::WorkDir),
            _ => Err(format!("Unknown kind = {}", string))
        }
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Section {
    pub kind: Kind,
    pub items: Vec<String>,
}

impl Section {
    pub fn from_lines<L:Borrow<str>>(lines: Vec<L>) -> (Option<Section>, Vec<L>) {
        let mut remaining_lines = Vec::new();
        let mut items = Vec::new();
        let mut kind: Option<Kind> = None;
        let mut end_reached = false;
        for line in lines {
            if end_reached {
                remaining_lines.push(line);
                continue;
            }
            if line.borrow().is_empty() {
                continue;
            }
            match Kind::from_str(line.borrow()) {
                Ok(new_kind) => {
                    if kind.is_some() {
                        end_reached = true;
                        remaining_lines.push(line);
                    } else {
                        kind = Some(new_kind);
                    }
                }
                Err(_) => {
                    if kind.is_some() {
                        items.push(line)
                    }
                }
            }
        }
        if let Some(kind) = kind {
            let items = items.into_iter().map(|i| i.borrow().to_string()).collect();
            let section = Section { kind: kind, items: items };
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
    let (section, lines) = Section::from_lines(lines);
    let section = section.unwrap();
    assert_eq!(section.kind, Kind::Os);
    assert_eq!(section.items.len(), 1);
    assert_eq!(section.items.first().unwrap(), "debian-8.2");
    let (section, _) = Section::from_lines(lines);
    let section = section.unwrap();
    assert_eq!(section.kind, Kind::From);
    assert_eq!(section.items.len(), 1);
    assert_eq!(section.items.first().unwrap(), "debian:jessie");
}
