use eframe::egui;
use rand::{RngExt, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;
use rodio::{DeviceSinkBuilder, Player, Source};
use std::num::NonZero;
use std::time::Duration;

struct BrownNoise {
    sample_rate: u32,
    leak_factor: f32,
    scale_factor: f32,
    previous: f32,
    max_val: f32,
    rng: Xoshiro256PlusPlus, // thread-safe rng
}

impl BrownNoise {
    fn new(
        sample_rate: u32,
        leak_factor: f32,
        scale_factor: f32,
    ) -> Self {
        let rng = Xoshiro256PlusPlus::from_rng(&mut rand::rng());
        Self {
            sample_rate,
            leak_factor,
            scale_factor,
            previous: 0.0,
            max_val: 1.0,
            rng,
        }
    }
}

impl Iterator for BrownNoise {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let white = self.rng.random_range(-1.0..1.0);
        let mut brown = self.previous * self.leak_factor + white * self.scale_factor;

        self.max_val = self.max_val.max(brown.abs());
        brown /= self.max_val;

        self.previous = brown;

        Some(brown)
    }
}

impl Source for BrownNoise {
    fn current_span_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> rodio::ChannelCount {
        NonZero::new(1).unwrap()
    }

    fn sample_rate(&self) -> rodio::SampleRate {
        NonZero::new(self.sample_rate).unwrap()
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

struct MyApp {
    leak_factor: f32,
    scale_factor: f32,
    playing: bool,
    sink: rodio::MixerDeviceSink,
    player: Option<Player>,
}

impl Default for MyApp {
    fn default() -> Self {
        let mut sink = DeviceSinkBuilder::open_default_sink()
            .expect("Unable to open default sink");
        sink.log_on_drop(false);
        Self {
            leak_factor: 0.9999,
            scale_factor: 0.01,
            playing: false,
            sink,
            player: None,
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("Brown Noise");
            ui.add(egui::Slider::new(&mut self.leak_factor, 0.0..=1.0).text("Leak Factor"));
            ui.add(egui::Slider::new(&mut self.scale_factor, 0.0..=1.0).text("Scale Factor"));
            ui.horizontal(|ui| {
                if ui.button("Reset").clicked() {
                    self.scale_factor = 0.01;
                    self.leak_factor = 0.9999;
                }
                if self.playing {
                    if ui.button("Stop").clicked() {
                        self.player = None;
                        self.playing = false;
                    }
                    ui.label("Playing!");
                } else {
                    if ui.button("Play").clicked() {
                        let source = BrownNoise::new(44100, self.leak_factor, self.scale_factor);
                        let player = Player::connect_new(self.sink.mixer());
                        player.append(source);
                        self.player = Some(player);
                        self.playing = true;
                    }
                    ui.label("Paused...");
                }
            });
        });
    }
}

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([250.0, 110.0]),
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
