use regex::Regex;
use toml::Value;

/// The custom mapping for a type
pub struct CustomMapping {
    pub the_type: String,
    pub regexes: Vec<Regex>,
}

read!(CustomMapping);

fn transform(content: &str) -> Vec<CustomMapping> {
    let mut custom_mapping = vec![];
    let value: Value = content.parse().unwrap();
    let tables = value.as_table().unwrap();

    for (the_type, regexes) in tables {
        let regexes = regexes
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap())
            .map(|s| Regex::new(s).unwrap())
            .collect();

        custom_mapping.push(CustomMapping {
            the_type: the_type.to_owned(),
            regexes,
        })
    }

    custom_mapping
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let mapping = super::transform(
            "# some comment
            SomeType=[\".*SomeType\"]
            UUID=[\".*[U]uid\"]
        ",
        );

        assert_eq!(2, mapping.len());
        assert_eq!(mapping[1].the_type, "UUID");
        assert_eq!(mapping[1].regexes.len(), 1);
        assert_eq!(mapping[1].regexes[0].as_str(), ".*[U]uid");
    }
}
