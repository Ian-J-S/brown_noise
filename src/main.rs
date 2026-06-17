use eframe::egui;

struct MyApp {
    leak_factor: f32,
    scale_factor: f32,
    playing: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            leak_factor: 0.9999,
            scale_factor: 0.01,
            playing: false,
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("Brown Noise");
            ui.add(egui::Slider::new(&mut self.leak_factor, 0.0..=1.0).text("Leak Factor"));
            ui.add(egui::Slider::new(&mut self.scale_factor, 0.0..=1.0).text("Scale Factor"));
            if ui.button("Reset").clicked() {
                self.scale_factor = 0.01;
                self.leak_factor = 0.9999;
            }
            if self.playing {
                if ui.button("Pause").clicked() {
                    self.playing = false;
                }
                ui.label("Pretend sound is playing");
            } else {
                if ui.button("Play").clicked() {
                    self.playing = true;
                }
            }
        });
    }
}

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([240.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Brown Noise",
        options,
        Box::new(|_cc| {
            Ok(Box::<MyApp>::default())
        }),
    )
}
