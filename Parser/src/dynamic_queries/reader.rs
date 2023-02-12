use crate::dynamic_queries::parse::PARAMETERIZED_IN_QUERY;
use grdb_orm_lib::dyn_query::DynamicQuery;
use grdb_orm_lib::toml::Value;

read!(Vec<DynamicQuery>);

/// Transforms a TOML file to a vec of [DynamicQuery]
fn transform(content: &str) -> Vec<DynamicQuery> {
    let value: Value = content.parse().unwrap();
    let tables = value.as_table().unwrap();
    let queries = tables
        .iter()
        .map(|(_, table)| grdb_orm_lib::toml::de::from_str(&table.to_string()).unwrap())
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
    use grdb_orm_lib::dyn_query::DynamicQuery;

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
            map_to_different_type: None,
            bypass_index_optimizer: false,
        };

        println!("{}", x.to_toml());
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
