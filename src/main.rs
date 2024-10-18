#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod models;
use dotenv::dotenv;
use eframe::egui::{self, Context};
use log::info;
use models::{Loglevel, WCSConfig, WWCPConfig, WizepassAuthConfig, WizepassConfig};

fn main() -> eframe::Result {
    dotenv().ok();
    pretty_env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    let mut wizepass_auth_config = WizepassAuthConfig {
        url: "".to_string(),
        instance_id: "".to_string(),
        rp_id: "".to_string(),
        cert_path: None,
    };

    let mut wwcp_config = WWCPConfig {
        loglevel: Loglevel("".to_string()),
    };
    let mut wcs_config = WCSConfig {
        loglevel: Loglevel("".to_string()),
    };

    eframe::run_simple_native("Wizepass config", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Create Wizepass config");

            ui.heading("Wizepass Config");
            ui.horizontal(|ui| {
                let url_label = ui.label("Wizepass url : ");
                ui.text_edit_singleline(&mut wizepass_auth_config.url)
                    .labelled_by(url_label.id);
            });

            ui.horizontal(|ui| {
                let instance_id_label = ui.label("Instance ID : ");
                ui.text_edit_singleline(&mut wizepass_auth_config.instance_id)
                    .labelled_by(instance_id_label.id);
            });

            ui.horizontal(|ui| {
                let rp_id_label = ui.label("RP ID : ");
                ui.text_edit_singleline(&mut wizepass_auth_config.rp_id)
                    .labelled_by(rp_id_label.id);
            });

            ui.horizontal(|ui| {
                let cert_path_label = ui.label("Certificate path (Optional) : ");
                let mut new_path = String::new();
                ui.text_edit_singleline(&mut new_path)
                    .labelled_by(cert_path_label.id);
                wizepass_auth_config.cert_path = Some(new_path);
            });

            ui.heading("Wizepass Credental Provider");
            ui.horizontal(|ui| {
                let wwcp_label = ui.label("Credential provider loglevel: ");
                ui.text_edit_singleline(&mut wwcp_config.loglevel.0)
                    .labelled_by(wwcp_label.id);
            });

            ui.heading("Wizepass Credental Service");
            ui.horizontal(|ui| {
                let wcs_label = ui.label("Credential service loglevel: ");
                ui.text_edit_singleline(&mut wcs_config.loglevel.0)
                    .labelled_by(wcs_label.id);
            });
            if ui.button("Create").clicked() {
                let wizepass_config = WizepassConfig {
                    wizepassauth: wizepass_auth_config.clone(),
                    wwcp: wwcp_config.clone(),
                    wcs: wcs_config.clone(),
                };
                create_toml(ctx.clone(), wizepass_config)
            }
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
