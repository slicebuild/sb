pub struct SemanticInfo {
    pub name: String,
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
    pub prerelease: i32,
    pub prerelease_major: i32,
    pub prerelease_minor: i32,
}

impl SemanticInfo {
    /// # Examples
    ///
    /// ```
    /// use sb::semantic_info_parse::SemanticInfo;
    /// let semantic_info = SemanticInfo::from_str("myapp-0.1");
    /// assert_eq!(semantic_info.name, "myapp");
    /// assert_eq!(semantic_info.major, 0);
    /// assert_eq!(semantic_info.minor, 1);
    /// assert_eq!(semantic_info.patch, 0);
    /// assert_eq!(semantic_info.prerelease, 0);
    /// assert_eq!(semantic_info.prerelease_major, 0);
    /// assert_eq!(semantic_info.prerelease_minor, 0);

    /// let semantic_info = SemanticInfo::from_str("0.1");
    /// assert!(semantic_info.name.is_empty());
    /// assert_eq!(semantic_info.major, 0);
    /// assert_eq!(semantic_info.minor, 1);
    /// assert_eq!(semantic_info.patch, 0);
    /// assert_eq!(semantic_info.prerelease, 0);
    /// assert_eq!(semantic_info.prerelease_major, 0);
    /// assert_eq!(semantic_info.prerelease_minor, 0);

    /// let semantic_info = SemanticInfo::from_str("0.1-beta.2");
    /// assert!(semantic_info.name.is_empty());
    /// assert_eq!(semantic_info.major, 0);
    /// assert_eq!(semantic_info.minor, 1);
    /// assert_eq!(semantic_info.patch, 0);
    /// assert_eq!(semantic_info.prerelease, -2);
    /// assert_eq!(semantic_info.prerelease_major, 2);
    /// assert_eq!(semantic_info.prerelease_minor, 0);

    /// let semantic_info = SemanticInfo::from_str("myapp-0.1-beta.2");
    /// assert_eq!(semantic_info.name, "myapp");
    /// assert_eq!(semantic_info.version, "0.1");
    /// assert_eq!(semantic_info.prerelease, "beta2");

    /// let semantic_info = SemanticInfo::from_str("my-app-0.1-beta.2");
    /// assert_eq!(semantic_info.name, "my-app");
    /// assert_eq!(semantic_info.version, "0.1");
    /// assert_eq!(semantic_info.prerelease, "beta.2");
    /// ```
    pub fn from_str(str: &str) -> SemanticInfo {
        SemanticInfo::from_string(str.to_string())
    }

    pub fn from_string(string: String) -> SemanticInfo {
        println!("{}", string);
        let dot_position = string.find('.').unwrap();
        if let Some(dash_position) = string.find('-') {
            if dash_position < dot_position {
                let (name, string) = SemanticInfo::extract_name_from_string(string);
                let (version, prerelease) =
                    SemanticInfo::extract_version_and_prerelease_from_string(string);
                SemanticInfo {
                    name: name,
                    version: version,
                    prerelease: prerelease,
                }
            } else {
                let (version, prerelease) =
                    SemanticInfo::extract_version_and_prerelease_from_string(string);
                SemanticInfo {
                    name: String::new(),
                    version: version,
                    prerelease: prerelease,
                }
            }
        } else {
            SemanticInfo {
                name: String::new(),
                version: string,
                prerelease: String::new(),
            }
        }
    }

    fn extract_name_from_string(string: String) -> (String, String) {
        let has_name = if let Some(dash_position) = string.find('-') {
            let dot_position = string.find('.').unwrap();
            dash_position < dot_position
        } else {
            false
        };

        if has_name {
            let mut name = String::new();
            let string = SemanticInfo::extract_next_part_of_name_from_string(&mut name, string);
            println!("Name = {}", name);
            (name, string)
        } else {
            (String::new(), string)
        }
    }

    fn extract_next_part_of_name_from_string(current_name: &mut String, string: String) -> String {
        let parts: Vec<&str> = string.splitn(2, '-').collect();
        assert_eq!(parts.len(), 2);
        current_name.push_str(parts.first().unwrap());
        let string = parts.last().unwrap().to_string();

        let has_next_part = if let Some(dash_position) = string.find('-') {
            let dot_position = string.find('.').unwrap();
            dash_position < dot_position
        } else {
            false
        };

        if has_next_part {
            current_name.push('-');
            SemanticInfo::extract_next_part_of_name_from_string(current_name, string)
        } else {
            string
        }
    }

    fn extract_version_and_prerelease_from_string(string: String) -> (String, String) {
        let parts: Vec<&str> = string.splitn(2, '-').collect();
        let version = parts.first().unwrap().to_string();
        if parts.len() == 1 {
            (version, String::new())
        } else {
            (version, parts.last().unwrap().to_string())
        }
    }
}
