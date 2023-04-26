#![cfg_attr(not(debug_assertions), windows_subsystem="windows")]
#![allow(non_snake_case)]
#![allow(unused_must_use)]
use Broken::*;

#[derive(Parser, Debug)]
#[command(author=Broken::Constants::AUTHOR, about=Broken::Constants::About::PocketSolar, version)]
pub struct Args {
    // Reset settings on boot
    #[arg(short, long, help = "Reset to default settings")]
    defaultSettings: bool,
}

// ----------------------------------------------------------------------------|

use egui::plot::Line;
use egui::plot::Plot;
use egui::plot::PlotPoints;
use egui::plot::Points;
use egui::Color32;

const BAUDRATE: u32 = 9600;

#[path = "PocketSolar/IVCurve.rs"]
mod IVCurve;

#[path = "PocketSolar/Serial.rs"]
mod Serial;

#[path = "PocketSolar/GUI.rs"]
mod GUI;

// ----------------------------------------------------------------------------|

#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Default)]
pub struct PocketSolarApp {
    // #[serde(skip)]
    solarPanelCurve: IVCurve::IVCurve,

    // Current, voltage amplification factor
    Ki: f64,
    Kv: f64,

    // Plot options
    plotPoints: bool,
    plotIVcurve: bool,
    plotPVcurve: bool,

    // Export Window
    showExportWindow: bool,
    exportNOfPoints: i64,
    outputCSV: String,

    // Serial
    #[serde(skip)]
    serialPort: Option<Box<dyn serialport::SerialPort>>,
    portName: String,

    // Other configurations
    showConfigurationWindow: bool,

    // Regression
    regressionSteps: i64,
    recalculateRegressionOnCoefficientChanges: bool,
}

impl PocketSolarApp {
    pub fn new(cc: &eframe::CreationContext<'_>, args: Args) -> PocketSolarApp {

        // Restore previous settings if any
        if let Some(storage) = cc.storage {
            if !args.defaultSettings {
                return eframe::get_value(storage, "PocketSolar").unwrap_or_default();
            }
        }

        // Default configuration
        return PocketSolarApp {

            // Current, voltage amplification factor
            Ki: 1.0,
            Kv: 10.0,

            // Plot options
            plotPoints: true,
            plotIVcurve: true,
            plotPVcurve: true,

            // Export
            exportNOfPoints: 20,

            // Serial
            portName: str!("None"),

            // Regression
            regressionSteps: 100,
            recalculateRegressionOnCoefficientChanges: false,

            ..PocketSolarApp::default()
        };
    }
}

// ----------------------------------------------------------------------------|

fn main() {
    Broken::setupLog();
    let args = Args::parse();

    // Compile NATIVELY
    #[cfg(not(target_arch = "wasm32"))]
    eframe::run_native("PocketSolar", eframe::NativeOptions::default(), Box::new(|cc| {
        let app = Box::new(PocketSolarApp::new(cc, args));
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        return app;
    }));
}
