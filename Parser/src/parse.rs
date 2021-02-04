use crate::configuration::Config;
use crate::dynamic_queries::DynQueryParser;
use crate::swift_struct::TableWriter;
use sqlite_parser::Metadata;

/// Starting point of parsing [Metadata] and [Config]
pub(crate) fn parse(tables: Metadata, config: Config) {
    // No tables? Something is wrong
    assert!(!tables.tables.is_empty());

    // Initialize the output dir
    let safe_output_dir = crate::output_dir_initializer::initialize(&config.output_dir);

    // Write the shared enum
    crate::shared::write(&config);

    // Write the tables
    TableWriter {
        tables: &tables,
        config: &config,
        safe_output_dir: safe_output_dir.clone(),
    }
    .write();

    // Write the dynamic queries
    DynQueryParser::new(&config, &tables).parse();

    // For the Swift code
    crate::format_swift_code::format_swift_code(&config, &safe_output_dir);
}
