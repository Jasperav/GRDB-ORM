use regex::Regex;
use toml::Value;

/// The configuration of a dynamic query
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct DynamicQuery {
    pub parameter_types: Vec<(String, String, String)>,
    pub extension: String,
    pub func_name: String,
    pub return_types: Vec<String>,
    pub return_types_is_array: bool,
    pub query: String,
}

read!(DynamicQuery);

/// Transforms a TOML file to a vec of [DynamicQuery]
fn transform(content: &str) -> Vec<DynamicQuery> {
    let value: Value = content.parse().unwrap();
    let tables = value.as_table().unwrap();
    let queries = tables
        .iter()
        .map(|(_, table)| toml::de::from_str(&table.to_string()).unwrap())
        .collect::<Vec<DynamicQuery>>();

    validate(&queries);

    queries
}

/// Validates [DynamicQuery]s for common problems
// Note sure how I can replace the regex
#[allow(clippy::trivial_regex)]
fn validate(queries: &[DynamicQuery]) {
    let regex = Regex::new(r" \?").unwrap();

    for dyn_query in queries {
        // Check if the amount of parameters query placeholders (?) equals the amount of Swift parameters
        let occurrences = regex
            .captures(&dyn_query.query)
            .map(|cap| cap.len())
            .unwrap_or(0);

        assert_eq!(dyn_query.parameter_types.len(), occurrences);

        // Check for a optional return type and if its an array (this is illegal)
        if dyn_query.return_types.len() == 1
            && dyn_query.return_types_is_array
            && dyn_query.return_types.iter().any(|r| r.contains('?'))
        {
            panic!("Return type is an optional array, this makes no sense")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dynamic_queries::reader::transform;
    use crate::dynamic_queries::DynamicQuery;

    /// Use this to check out how various configurations are presented in TOML
    #[test]
    fn spitout_example_toml() {
        let x = DynamicQuery {
            extension: "User".to_string(),
            func_name: "findByUsername".to_string(),
            parameter_types: vec![(
                "User".to_string(),
                "firstName".to_string(),
                "firstName".to_string(),
            )],
            return_types: vec!["User".to_string()],
            return_types_is_array: true,
            query: "select * from User where firstName = ?".to_string(),
        };

        println!("{}", toml::ser::to_string(&x).unwrap());
    }

    #[test]
    #[should_panic]
    fn test_incorrect_parameter_types() {
        transform(
            "
            [first]
            extension = \"\"
            func_name = \"\"
            parameter_types = []
            return_types = []
            return_types_is_array = true
            query = \"select * from Book order by ?\"
        ",
        );
    }
}
