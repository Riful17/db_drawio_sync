#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui_logger;

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
                log::info!("Info button Run pressed {} {} {}", self.db_conn, self.table_header, self.table_element);
            }
        });

        egui::Window::new("Log").show(ctx, |ui| {
            egui_logger::logger_ui(ui);
        });
    }
}