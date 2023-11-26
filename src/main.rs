#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui_logger;
use mysql::*;
use mysql::prelude::*;

#[derive(Debug, PartialEq, Eq)]
struct TableStructure {
    field: String,
    // r#type: Option<String>,
    collation: String,
    // null: Option<String>,
    // key: Option<String>,
    // default: Option<String>,
    // extra: Option<String>,
    // privileges: Option<String>,
    // comment: Option<String>,
}

#[derive(Default)]
struct MyApp {
    db_conn: String,
    table_header: String,
    table_element: String,
}

fn main() {

    egui_logger::init_with_max_level(log::LevelFilter::Debug).expect("Error initializing logger");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().
            with_inner_size([460.0, 240.0]).
            with_position([400.0,400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "DB DrawIO Sync",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    ).unwrap();
}


impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("DB conn");
            ui.horizontal(|ui| {
                let db_conn_label = ui.label("DB conn: ");
                ui.text_edit_singleline(&mut self.db_conn)
                    .labelled_by(db_conn_label.id);
            });
            ui.horizontal(|ui| {
                let table_header_label = ui.label("Table header: ");
                ui.text_edit_multiline(&mut self.table_header)
                    .labelled_by(table_header_label.id);
            });
            ui.horizontal(|ui| {
                let table_element_label = ui.label("Table element: ");
                ui.text_edit_multiline(&mut self.table_element)
                    .labelled_by(table_element_label.id);
            });
            if ui.button("Run").clicked() {
                db_query().unwrap();
                log::info!("Info button Run pressed {} {} {}", self.db_conn, self.table_header, self.table_element);
            }
        });

        egui::Window::new("Log").show(ctx, |ui| {
            egui_logger::logger_ui(ui);
        });
    }
}


pub fn db_query() -> std::result::Result<(), Box<dyn std::error::Error>> {
    log::trace!("db_query ...");
    let url = "mysql://root:some_dev_password@94.19.108.70:3306/catalog";
    let pool = Pool::new(url)?;

    log::trace!("pool.get_conn ...");
    let mut conn = pool.get_conn()?;
    log::trace!("pool.get_conn done");

    log::trace!("conn.query: {}", "SHOW DATABASES;");
    let dbs: Vec<String> = conn.query("SHOW DATABASES;")?;
    log::info!("Databases: {:?}", dbs);

    for db_name in dbs {
        log::trace!("db: {}", db_name);
        log::trace!("SELECT TABLE_NAME FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_SCHEMA = '{}'", db_name);
        let tables: Vec<String> = conn.query(format!("SELECT TABLE_NAME FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_SCHEMA = '{}'", db_name))?;
        log::info!("Database: {} Tables {:?}", db_name, tables);
        for table_name in tables {
            log::warn!("NOT IMPLEMENTED SHOW FULL COLUMNS FROM {}.{};",db_name, table_name);
            // let fields = conn
            //     .query_map(format!("SHOW FULL COLUMNS FROM {}.{}",db_name, table_name),
            //                |(field, r#type, collation, null, key, default, extra, privileges, comment)| {
            //     TableStructure { field, r#type, collation, null, key, default, extra, privileges , comment  }
            // })?;

            // for field in fields {
            //     log::info!("{}.{}.{}",db_name, table_name, field.field);
            // }
        }
    }


    // let val: Option<String> = conn.query_first("SELECT TABLE_NAME FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_SCHEMA = 'catalog'")?;
    // log::info!("{:?}", val);
    //
    // let val: Option<String> = conn.query_first("SHOW FULL COLUMNS FROM catalog.manager;")?;
    // log::info!("{:?}", val);

    // wrong struct for mappings
    // let selected_payments = conn
    //     .query_map(
    //         "SELECT * from devices",
    //         |(title, uuid, state)| {
    //            Devices { title, uuid, state }
    //         },
    //     )?;
    //
    // for dev in selected_payments {
    //     log::info!("{:?} {:?} {:?}", dev.title, dev.uuid, dev.state);
    // }

    log::info!("db_query done");

    Ok(())
}

