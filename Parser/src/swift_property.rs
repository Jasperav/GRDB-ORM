/// This file contains methods that makes it convenient to write Swift properties
use sqlite_parser::{Column, Table, Type};

use crate::configuration::Config;
use crate::custom_mapping::CustomMapping;

/// This represents a property in Swift
/// This is an extracted column in SQLite
#[derive(Clone)]
pub struct SwiftProperty {
    // Currently always the same as the SQLite column name, maybe in the future this can be changed
    // to convert between camelCase and snake_case.
    // Currently, the column names are exactly the same as the Swift property.
    pub swift_property_name: String,
    pub swift_type: SwiftTypeWithTypeName,
    // The actual column from which the data is extracted
    pub column: Column,
}

/// Type information about the Swift property
#[derive(Clone)]
pub struct SwiftTypeWithTypeName {
    /// The type name of the Swift property (can be customized by the custom_mapping.toml file)
    pub type_name: String,
    pub swift_type: SwiftType,
}

/// A type can either be Json or not Json. Simple, right?
#[derive(Copy, Clone)]
pub enum SwiftType {
    NoJson,
    Json,
}

impl SwiftType {
    pub fn from_str(str: &str) -> Self {
        // If the type is build in, it's not Json. Else it must be Json.
        if is_build_in_type(str) {
            Self::NoJson
        } else {
            Self::Json
        }
    }
}

/// Transforms a given [Column] to a [SwiftProperty], taking into account the custom mapping
pub fn create_swift_property(column: &Column, custom_mapping: &[CustomMapping]) -> SwiftProperty {
    // This is the type at the beginning
    // It can be changes by the custom_mapping
    let mut inferred_type = sqlite_type_to_swift_type(column.the_type).to_string();

    // Check if the user wants to do a custom transformation
    for mapping in custom_mapping {
        if mapping
            .regexes
            .iter()
            .any(|regex| regex.is_match(&column.name))
        {
            // Found a match, change the inferred type and stop checking for other matches, since
            // the matching is done from top to bottom
            inferred_type = mapping.the_type.clone();

            break;
        }
    }

    let swift_type = SwiftType::from_str(&inferred_type);

    // If the column is nullable, also make the Swift property nullable
    let type_name = if column.nullable {
        inferred_type + "?"
    } else {
        inferred_type
    };

    SwiftProperty {
        // Currently always the same as the SQLite column name
        swift_property_name: column.name.clone(),
        swift_type: SwiftTypeWithTypeName {
            type_name,
            swift_type,
        },
        column: column.clone(),
    }
}

/// Calls [create_swift_property] for all columns in the provided [Table]
pub fn create_swift_properties(
    table: &Table,
    custom_mapping: &[CustomMapping],
) -> Vec<SwiftProperty> {
    table
        .columns
        .iter()
        .map(|column| create_swift_property(column, custom_mapping))
        .collect()
}

/// This trait makes it easy to process decoding Swift properties from a [GRDB.Row]
pub trait SwiftPropertyDecoder {
    /// Returns the current index which will be used to extract the property from
    fn row_index(&self) -> String;
    /// Describes how to assign the result of the decoded Swift property
    fn assign(&self, property_name: &str, decoded: &str) -> String;
}

/// Decodes a [SwiftProperty]
/// This is the process of turning a [GRDB.Row] index to a Swift property
pub fn decode_swift_property<T: SwiftPropertyDecoder>(
    decoder: &T,
    property: &SwiftProperty,
) -> String {
    let row_index = decoder.row_index();
    // This is the correct row index for decoding the SwiftProperty
    let row = format!("row[{}]", row_index);

    match property.swift_type.swift_type {
        SwiftType::NoJson => {
            // This is easy, just directly assign it
            decoder.assign(&property.swift_property_name, &row)
        }
        SwiftType::Json => {
            // Json is a little tricky, especially dealing with nullable columns
            // Force unwrapping should be fine (if the user doesn't do anything weird with the database)
            let decode = format!(
                "try! Shared.jsonDecoder.decode({}.self, from: {})",
                &property.swift_type.type_name, row
            );

            let decode = if property.column.nullable {
                // Wrap it inside if let else block and remove the optional type
                format!(
                    "{{
                        if row.hasNull(atIndex: {}) {{
                            return nil
                        }} else {{
                            return {}
                        }}
                    }}()",
                    row_index,
                    // Remove the optional type
                    decode.replace('?', "")
                )
            } else {
                decode
            };

            // Now the property can be assigned
            decoder.assign(&property.swift_property_name, &decode)
        }
    }
}

/// A type is 'build-in' when the type is standard Swift type
pub fn is_build_in_type(check: &str) -> bool {
    check == "String"
        || check == "Int"
        || check == "UUID"
        || check == "Int64"
        || check == "Int32"
        || check == "Bool"
}

/// Creates a Swift type from a [&str]
pub fn create_swift_type_name(from: &str, config: &Config) -> String {
    if is_build_in_type(from) {
        // If the type is build in, nothing has to be done
        return from.to_string();
    }

    // Capitalize the input
    let struct_name_before_fix = crate::some_kind_of_uppercase_first_letter(from);

    // Take into account the prefix and suffix
    format!(
        "{}{}{}",
        &config.prefix_swift_structs, struct_name_before_fix, &config.suffix_swift_structs
    )
}

/// Translates a SQLite type to a Swift type
fn sqlite_type_to_swift_type(t: Type) -> &'static str {
    match t {
        Type::Text | Type::String | Type::Real => "String",
        Type::Integer => "Int",
        Type::Blob => "Data",
    }
}

/// This transforms given [SwiftProperty]s to SQLite database values
pub fn swift_properties_to_sqlite_database_values(swift_properties: &[&SwiftProperty]) -> String {
    swift_properties
        .iter()
        .map(|property| {
            let is_optional = property.swift_type.type_name.contains('?');
            let removed_optional = property.swift_type.type_name.replace('?', "");

            match property.swift_type.swift_type {
                SwiftType::NoJson => {
                    let database_value = if removed_optional == "UUID" {
                        // Currently always the uuid string property is used, no data
                        ".uuidString"
                    } else {
                        ""
                    };

                    let db_value = property.swift_property_name.clone() + database_value;

                    if is_optional {
                        // Only remove the first dot, because else optional uuid's will result in compile errors
                        db_value.replacen('.', "?.", 1)
                    } else {
                        db_value
                    }
                }
                SwiftType::Json => {
                    // This is a bit ugly, since optionals needs to be handled as well
                    let (head, tail, variable_name) = if is_optional {
                        let head = format!("try {}.map {{", property.swift_property_name);
                        let tail = "}";

                        (head, tail, "$0".to_string())
                    } else {
                        // Both empty, maybe some cleaner way to do this would be nice
                        ("".to_string(), "", property.swift_property_name.clone())
                    };

                    format!(
                        "{{
                            {}
                                let data = try Shared.jsonEncoder.encode({})
                                return String(data: data, encoding: .utf8)!{}
                            }}()",
                        head, variable_name, tail
                    )
                }
            }
        })
        .collect::<Vec<_>>()
        .join(", \n")
}

/// Creates a property: Type from a swift property
pub fn swift_property_and_type(property: &SwiftProperty) -> String {
    format!(
        "{}: {}",
        &property.swift_property_name, &property.swift_type.type_name
    )
}

/// Convenience method to create an property: Type from swift properties
pub fn swift_properties_and_types(properties: &[&SwiftProperty]) -> Vec<String> {
    properties
        .iter()
        .map(|p| swift_property_and_type(p))
        .collect()
}
