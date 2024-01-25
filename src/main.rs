#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

struct showExit{
    showExits: bool,
}
fn main() -> Result<(), eframe::Error> {
    let mut newShowExit = showExit{
        showExits: false
    };
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    // Our application state:
    let mut name = "Arthur".to_owned();
    let mut age = 42;

    eframe::run_simple_native("My egui App", options, move |ctx, frame| {
        if ctx.input(|i| i.key_down(egui::Key::A)){
            newShowExit.showExits= true;
        }
        else{
            newShowExit.showExits= false;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if  newShowExit.showExits==true{

            ui.heading("You pressed A");
            }
            else {
                ui.heading("Press A");
            }
            ui.horizontal(|ui| {

                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut age, 0..=140).text("age"));
            if ui.button("Click each year").clicked() {
                age += 1;
            }
            ui.label(format!("Hello '{name}', age {age}"));
        });
    })
}

