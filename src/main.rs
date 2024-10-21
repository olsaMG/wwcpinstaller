#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod models;

use dotenv::dotenv;
use eframe::egui::{self, Context, Vec2};
use log::info;
use models::{Loglevel, TracingLevel, WCSConfig, WWCPConfig, WizepassAuthConfig, WizepassConfig};

fn main() -> eframe::Result {
    dotenv().ok();
    pretty_env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([420.0, 340.0]),
        ..Default::default()
    };

    let mut wizepass_auth_config = WizepassAuthConfig {
        url: "".to_string(),
        instance_id: "".to_string(),
        rp_id: "".to_string(),
        cert_path: None,
    };

    let mut loglevel = TracingLevel::Info;

    let mut wwcp_config = WWCPConfig {
        loglevel: Loglevel("".to_string()),
    };
    let mut wcs_config = WCSConfig {
        loglevel: Loglevel("".to_string()),
    };

    eframe::run_simple_native("Wizepass config", options, move |ctx, _frame| {
        // ctx.set_theme(Theme::Light);
        egui_extras::install_image_loaders(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.heading("Create Wizepass config");
                ui.separator();

                ui.add_space(10.0);
                ui.add(
                    egui::Image::new(egui::include_image!("../assets/logo.png"))
                        .max_size(Vec2::new(100.0, 100.0))
                        .rounding(100.0),
                );

                ui.add_space(10.0);
            });

            egui::Grid::new("config")
                .num_columns(2)
                .spacing([8.0, 10.0])
                .striped(true)
                .show(ui, |ui| {
                    let url = ui.label("Wizepass url: ");
                    ui.text_edit_singleline(&mut wizepass_auth_config.url)
                        .labelled_by(url.id);
                    ui.end_row();

                    let instance_id = ui.label("Instance ID: ");
                    ui.text_edit_singleline(&mut wizepass_auth_config.instance_id)
                        .labelled_by(instance_id.id);
                    ui.end_row();

                    let rp_id = ui.label("RP ID: ");
                    ui.text_edit_singleline(&mut wizepass_auth_config.rp_id)
                        .labelled_by(rp_id.id);
                    ui.end_row();

                    let mut new_path = String::new();

                    let cert_path = ui.label("Certificate path (Optional): ");
                    ui.text_edit_singleline(&mut new_path)
                        .labelled_by(cert_path.id);
                    wizepass_auth_config.cert_path = Some(new_path);
                    ui.end_row();

                    ui.label("Loglevel: ");
                    egui::ComboBox::from_id_salt("Loglevel")
                        .selected_text(format!("{:?}", loglevel))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut loglevel, TracingLevel::Info, "Info");
                            ui.selectable_value(&mut loglevel, TracingLevel::Debug, "Debug");
                            ui.selectable_value(&mut loglevel, TracingLevel::Error, "Error");
                            ui.selectable_value(&mut loglevel, TracingLevel::Warn, "Warn");
                            ui.selectable_value(&mut loglevel, TracingLevel::Trace, "Trace");
                        });

                    loglevel.set_loglevel(&mut wwcp_config, &mut wcs_config);
                });
        });
        egui::TopBottomPanel::new(egui::panel::TopBottomSide::Bottom, "buttons")
            .exact_height(50.0)
            .show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    if ui.button("Create").clicked() {
                        let wizepass_config = WizepassConfig {
                            wizepassauth: wizepass_auth_config.clone(),
                            wwcp: wwcp_config.clone(),
                            wcs: wcs_config.clone(),
                        };
                        create_toml(ctx.clone(), wizepass_config)
                    }
                });
            });
    })
}

fn create_toml(ctx: Context, wizepass_config: WizepassConfig) {
    info!("Creating toml file");
    let toml = toml::to_string(&wizepass_config).unwrap();

    std::fs::write("config.toml", toml).expect("Error writing file");

    info!("Exiting program");

    std::thread::spawn(move || {
        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
    });
}
