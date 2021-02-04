mod main_struct;
mod primary_key;

use crate::line_writer::LineWriter;

pub(crate) use main_struct::QueryWriterMainStruct;
pub(crate) use primary_key::QueryWriterPrimaryKey;

type WriteResult = (&'static str, String);

fn write_static_queries(line_writer: &mut LineWriter, queries: Vec<WriteResult>) {
    line_writer.add_comment("Static queries");

    for (property, query) in queries {
        line_writer.add_with_modifier(format!("static let {} = \"{}\"", property, query));
    }

    line_writer.new_line();
}
