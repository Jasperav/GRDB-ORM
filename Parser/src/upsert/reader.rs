use toml::Value;

/// The custom mapping for a type
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Upsert {
    pub table: String,
    pub columns_to_update: Vec<String>,
    pub func_name: String,
}

read!(Upsert);

fn transform(content: &str) -> Vec<Upsert> {
    let value: Value = content.parse().unwrap();
    let tables = value.as_table().unwrap();

    tables
        .iter()
        .map(|(_, table)| toml::de::from_str(&table.to_string()).unwrap())
        .collect()
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let mut upsert = super::transform(
            "
            [user-first-name]
            table = \"user\"
            columns_to_update = [\"firstName\"]
            func_name = \"method_name\"
        ",
        );

        assert_eq!(1, upsert.len());

        let upsert = upsert.remove(0);

        assert_eq!("method_name", upsert.func_name);
        assert_eq!("user", upsert.table);
        assert_eq!(1, upsert.columns_to_update.len());
        assert_eq!("firstName", upsert.columns_to_update[0]);
    }
}
