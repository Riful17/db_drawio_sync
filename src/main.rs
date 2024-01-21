mod data;

use chrono::Local;
use log::*;
use sqlparser::dialect;
use sqlparser::parser::Parser;
use std::fs::File;
use std::io::{Write};
use xmltree::Element;


fn main() {

    let names_element = Element::parse(data::DRAW_IO_TEST_DATA.as_bytes()).unwrap();

    println!("{:#?}", names_element);
    {
        // get first `name` element
        // let name = names_element
        //     .get_mut_child("name")
        //     .expect("Can't find name element");
        // name.attributes.insert("suffix".to_owned(), "mr".to_owned());
    }

    names_element
        .write_with_config(
            File::create("../result.drawio").unwrap(),
            xmltree::EmitterConfig::new()
                .write_document_declaration(false)
                .perform_indent(true),
        )
        .expect("TODO: panic message");

    let target = Box::new(File::create("app.log").expect("Can't create file"));

    env_logger::Builder::new()
        .target(env_logger::Target::Pipe(target))
        .filter(None, LevelFilter::Debug)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {} {}:{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args(),
            )
        })
        .init();

    let dialect = dialect::GenericDialect {};
    let res = Parser::parse_sql(&dialect, data::SQL_COMMENT).unwrap();

    for statement in res {
        if let sqlparser::ast::Statement::CreateTable { name, columns, .. } = statement {
            for each_name in name.0 {
                debug!("Table each_name: {:?}", each_name.value);
            }
            // debug!("Table: {:?}", name);
            for column_def in columns {
                debug!(
                    "Column: {}, Type: {}",
                    column_def.name.value, column_def.data_type
                );
                for opt in &column_def.options {
                    if opt.name.is_none() {
                        continue;
                    }
                    debug!(
                        "  Option: {:?} = {:?}",
                        opt.name.clone().unwrap().value,
                        opt.option
                    );
                }
            }
        }
    }
}
