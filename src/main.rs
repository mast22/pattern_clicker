#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::{DateTime, Datelike, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Timelike};
use egui_inbox::{UiInbox, UiInboxSender};
use enigo::{Button, Direction, Enigo, Mouse, Settings};
use std::thread;

use eframe::egui;

fn clicker(
    hour: u32,
    minute: u32,
    second: u32,
    milli: u32,
    tx: UiInboxSender<Option<DateTime<Local>>>,
) {
    let dt = Local::now();

    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    let naive_datetime = NaiveDateTime::new(
        NaiveDate::from_ymd_opt(dt.year(), dt.month(), dt.day()).unwrap(),
        NaiveTime::from_hms_milli_opt(hour, minute, second, milli).unwrap(),
    );
    let target_datetime: DateTime<Local> = Local.from_local_datetime(&naive_datetime).unwrap();

    loop {
        if Local::now() >= target_datetime {
            enigo.button(Button::Left, Direction::Click).unwrap();
            tx.send(Some(Local::now())).ok();
            break;
        }
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([240.0, 160.0]),
        ..Default::default()
    };
    let inbox = UiInbox::new();
    let mut state: Option<DateTime<Local>> = None;

    let dt = Local::now();
    let mut set_hour = dt.hour();
    let mut set_minute = dt.minute();
    let mut set_second = dt.second();
    let mut set_milli = 0;

    eframe::run_simple_native("Кликер", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(last) = inbox.read(ui).last() {
                state = last;
            }

            ui.heading("Выбери время для клика");
            egui::Grid::new("some_unique_id")
                .striped(true)
                .show(ui, |ui: &mut egui::Ui| {
                    ui.label("Час:");
                    ui.add(egui::DragValue::new(&mut set_hour).range(0..=23));
                    ui.end_row();

                    ui.label("Минута:");
                    ui.add(egui::DragValue::new(&mut set_minute).range(0..=59));
                    ui.end_row();

                    ui.label("Секунда:");
                    ui.add(egui::DragValue::new(&mut set_second).range(0..=59));
                    ui.end_row();

                    ui.label("Миллисекунда:");
                    ui.add(egui::DragValue::new(&mut set_milli).range(0..=999));
                    ui.end_row();

                    if ui.add(egui::Button::new("Задать клик")).clicked() {
                        let tx = inbox.sender();

                        thread::spawn(move || {
                            clicker(set_hour, set_minute, set_second, set_milli, tx)
                        });
                    }
                    ui.end_row();

                    match state {
                        Some(datetime) => {
                            let formatted = datetime.format("%H:%M:%S%.3f").to_string();

                            ui.label(format!("Выполнен: {:?}", formatted));
                            ui.end_row();
                        }
                        None => {
                            ui.label("Клик еще не выполнен");
                            ui.end_row();
                        }
                    }
                });
        });
    })
}
