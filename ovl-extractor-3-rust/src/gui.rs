use eframe::egui;
use std::path::PathBuf;
use rfd::FileDialog;
use std::fs::File;
use crate::ovl::OVLFile;
use anyhow::Result;

pub struct OVLExtractorApp {
    selected_file: Option<PathBuf>,
    sections: Vec<String>,
    status_message: String,
    error_message: Option<String>,
}

impl Default for OVLExtractorApp {
    fn default() -> Self {
        Self {
            selected_file: None,
            sections: Vec::new(),
            status_message: String::from("Ready"),
            error_message: None,
        }
    }
}

impl OVLExtractorApp {
    fn load_ovl_file(&mut self, path: PathBuf) -> Result<()> {
        let mut file = File::open(&path).map_err(|e| {
            anyhow::anyhow!("Failed to open file: {}", e)
        })?;

        let ovl = OVLFile::read(&mut file)?;

        self.sections = ovl.sections.iter()
            .map(|s| format!("{} ({} bytes)", s.name, s.size))
            .collect();

        Ok(())
    }

    fn extract_sections(&self, output_dir: PathBuf) -> Result<()> {
        let mut file = File::open(&self.selected_file.as_ref().unwrap()).map_err(|e| {
            anyhow::anyhow!("Failed to open file: {}", e)
        })?;

        let ovl = OVLFile::read(&mut file).map_err(|e| {
            anyhow::anyhow!("Failed to parse OVL file: {}", e)
        })?;

        std::fs::create_dir_all(&output_dir).map_err(|e| {
            anyhow::anyhow!("Failed to create output directory: {}", e)
        })?;

        for section in ovl.sections {
            let section_path = output_dir.join(format!("{}.bin", section.name));
            std::fs::write(&section_path, section.data).map_err(|e| {
                anyhow::anyhow!("Failed to write section {}: {}", section.name, e)
            })?;
        }

        Ok(())
    }
}

impl eframe::App for OVLExtractorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("OVL Extractor 3.0");
            
            // File selection
            if ui.button("Open OVL File").clicked() {
                if let Some(path) = FileDialog::new()
                    .add_filter("OVL files", &["ovl"])
                    .pick_file() 
                {
                    self.selected_file = Some(path.clone());
                    match self.load_ovl_file(path) {
                        Ok(_) => {
                            self.status_message = "File opened".to_string();
                            self.error_message = None;
                        }
                        Err(e) => {
                            self.error_message = Some(e.to_string());
                            self.sections.clear();
                        }
                    }
                }
            }

            // Display status messages
            if let Some(error) = &self.error_message {
                ui.colored_label(egui::Color32::RED, error);
            } else {
                ui.colored_label(egui::Color32::GREEN, &self.status_message);
            }
            
            if let Some(path) = &self.selected_file {
                ui.label(format!("Selected file: {}", path.display()));
                
                // Only show sections and extract button if there's no error
                if self.error_message.is_none() {
                    // Section list
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for section in &self.sections {
                            ui.label(section);
                        }
                    });
                    
                    // Extract button
                    if ui.button("Extract All").clicked() {
                        if let Some(dir) = FileDialog::new().pick_folder() {
                            match self.extract_sections(dir) {
                                Ok(_) => {
                                    self.status_message = "Extraction completed successfully".to_string();
                                    self.error_message = None;
                                }
                                Err(e) => {
                                    self.error_message = Some(e.to_string());
                                }
                            }
                        }
                    }
                }
            }
            
            // Status bar
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.label(&self.status_message);
            });
        });
    }
}

pub fn run_gui() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };
    
    eframe::run_native(
        "OVL Extractor 3.0",
        options,
        Box::new(|_cc| Box::new(OVLExtractorApp::default()))
    )
}