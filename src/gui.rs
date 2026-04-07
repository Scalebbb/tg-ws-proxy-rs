//! GUI module using egui for the proxy application

use std::sync::Arc;
use parking_lot::RwLock;
use eframe::egui;

#[derive(Clone)]
pub struct ProxyStats {
    pub active_connections: usize,
    pub total_connections: usize,
    pub bytes_sent: u64,
    pub bytes_received: u64,
}

impl Default for ProxyStats {
    fn default() -> Self {
        Self {
            active_connections: 0,
            total_connections: 0,
            bytes_sent: 0,
            bytes_received: 0,
        }
    }
}

pub struct ProxyApp {
    pub config_display: String,
    pub tg_link: String,
    pub stats: Arc<RwLock<ProxyStats>>,
    pub log_messages: Arc<RwLock<Vec<String>>>,
    pub is_running: bool,
}

impl ProxyApp {
    pub fn new(
        config_display: String,
        tg_link: String,
        stats: Arc<RwLock<ProxyStats>>,
        log_messages: Arc<RwLock<Vec<String>>>,
    ) -> Self {
        Self {
            config_display,
            tg_link,
            stats,
            log_messages,
            is_running: true,
        }
    }
}

impl eframe::App for ProxyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🚀 Telegram MTProto WS Bridge Proxy");
            ui.separator();

            // Status section
            ui.horizontal(|ui| {
                ui.label("Status:");
                if self.is_running {
                    ui.colored_label(egui::Color32::GREEN, "● Running");
                } else {
                    ui.colored_label(egui::Color32::RED, "● Stopped");
                }
            });

            ui.add_space(10.0);

            // Configuration section
            ui.group(|ui| {
                ui.label(egui::RichText::new("Configuration").strong());
                ui.add_space(5.0);
                ui.label(&self.config_display);
            });

            ui.add_space(10.0);

            // Telegram link section
            ui.group(|ui| {
                ui.label(egui::RichText::new("Telegram Proxy Link").strong());
                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.tg_link.as_str());
                    if ui.button("📋 Copy").clicked() {
                        ui.output_mut(|o| o.copied_text = self.tg_link.clone());
                    }
                });
            });

            ui.add_space(10.0);

            // Statistics section
            let stats = self.stats.read();
            ui.group(|ui| {
                ui.label(egui::RichText::new("Statistics").strong());
                ui.add_space(5.0);
                egui::Grid::new("stats_grid")
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("Active Connections:");
                        ui.label(stats.active_connections.to_string());
                        ui.end_row();

                        ui.label("Total Connections:");
                        ui.label(stats.total_connections.to_string());
                        ui.end_row();

                        ui.label("Bytes Sent:");
                        ui.label(format_bytes(stats.bytes_sent));
                        ui.end_row();

                        ui.label("Bytes Received:");
                        ui.label(format_bytes(stats.bytes_received));
                        ui.end_row();
                    });
            });

            ui.add_space(10.0);

            // Log section
            ui.group(|ui| {
                ui.label(egui::RichText::new("Recent Logs").strong());
                ui.add_space(5.0);
                
                egui::ScrollArea::vertical()
                    .max_height(200.0)
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        let logs = self.log_messages.read();
                        for msg in logs.iter().rev().take(100).rev() {
                            ui.label(msg);
                        }
                    });
            });
        });
    }
}

fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}
