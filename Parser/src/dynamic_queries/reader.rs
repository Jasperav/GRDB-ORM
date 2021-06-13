use crate::dynamic_queries::parse::PARAMETERIZED_IN_QUERY;
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
fn validate(queries: &[DynamicQuery]) {
    for dyn_query in queries {
        let illegal_patterns = [" in ?", " in (?)"];

        for illegal in &illegal_patterns {
            assert!(
                !dyn_query.query.contains(illegal),
                "use ' in {}'",
                PARAMETERIZED_IN_QUERY
            );
        }

        assert!(dyn_query.return_types.iter().all(|r| !r.contains('?')));

        // Check if the amount of parameters query placeholders (?) equals the amount of Swift parameters
        let occurrences_question_mark = dyn_query.query.matches('?').count();
        let occurrences_parameterized_query =
            dyn_query.query.matches(PARAMETERIZED_IN_QUERY).count();

        assert_eq!(
            dyn_query.parameter_types.len(),
            occurrences_question_mark + occurrences_parameterized_query,
            "Query: {}",
            dyn_query.query
        );

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
