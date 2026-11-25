//! Android host for the original CADCraft engine and egui front end.
//! This crate intentionally reuses CadApp and Session; it is not a replacement CAD engine.

#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

use cadcraft_engine::Session;
use cadcraft_ui_egui::{CadApp, Services};

struct AndroidShell(CadApp);

impl eframe::App for AndroidShell {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.0.logic(ctx);
    }

    fn raw_input_hook(&mut self, ctx: &egui::Context, raw: &mut egui::RawInput) {
        self.0.raw_input_hook(ctx, raw);
        cadcraft_ui_egui::cmdline::capture_tab(ctx, raw);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.0.ui(ui);
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        storage.set_string(cadcraft_ui_egui::options::PROFILE_KEY, self.0.profile_json());
        storage.set_string(cadcraft_ui_egui::PREFS_KEY, self.0.prefs_json());
    }

    fn persist_egui_memory(&self) -> bool {
        false
    }
}

/// Install the same format codecs used by the desktop and web hosts.
/// Android file picking is deliberately left to the SAF bridge in a later step.
fn install_io() {
    cadcraft_engine::cmd::file::set_io(cadcraft_engine::cmd::file::IoHooks {
        read: |bytes, name| cadcraft_io::read(bytes, name).map_err(|e| e.to_string()),
        write: |doc, name| cadcraft_io::write(doc, name).map_err(|e| e.to_string()),
        plot: Some(|doc, space, options| {
            cadcraft_io::plotter::plot(doc, space, options).map_err(|e| e.to_string())
        }),
    });
    cadcraft_engine::cmd::file::set_image_decoder(|bytes| {
        cadcraft_io::raster::decode(bytes).map_err(|e| e.to_string())
    });
    cadcraft_engine::cmd::file::set_write_as(|doc, name, options| {
        cadcraft_io::write_with(
            doc,
            name,
            &cadcraft_io::SaveOptions {
                version: options.version,
                binary: options.binary,
            },
        )
        .map_err(|e| e.to_string())
    });
    cadcraft_engine::cmd::file::set_entity_codec(cadcraft_engine::cmd::file::EntityCodec {
        groups: cadcraft_io::entity_groups,
        entity: cadcraft_io::entity_from_groups,
    });
    cadcraft_engine::cmd::file::set_object_records(cadcraft_io::object_records);
    cadcraft_engine::cmd::file::set_proxies_lost(cadcraft_io::proxies_lost);
    cadcraft_engine::cmd::file::set_dwg_compat_lost(cadcraft_io::dwg_compat_lost);
}

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
pub fn android_main(app: winit::platform::android::activity::AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );
    let startup = std::time::Instant::now();
    log::info!("Starting CADCraft Android host");
    install_io();
    log::info!("CADCraft startup: I/O codecs installed in {} ms", startup.elapsed().as_millis());

    let options = eframe::NativeOptions {
        android_app: Some(app),
        ..Default::default()
    };

    let result = eframe::run_native(
        "CADCraft",
        options,
        Box::new(|cc| {
            let phase = std::time::Instant::now();
            let session = Session::new();
            log::info!("CADCraft startup: Session::new completed in {} ms", phase.elapsed().as_millis());

            let phase = std::time::Instant::now();
            let mut cad = CadApp::new(session, Services::default());
            cad.system_languages = vec!["ar".to_owned()];
            log::info!("CADCraft startup: CadApp constructed in {} ms", phase.elapsed().as_millis());

            let phase = std::time::Instant::now();
            if let Some(storage) = cc.storage {
                if let Some(profile) = storage.get_string(cadcraft_ui_egui::options::PROFILE_KEY) {
                    cad.load_profile(&profile);
                }
                if let Some(prefs) = storage.get_string(cadcraft_ui_egui::PREFS_KEY) {
                    cad.load_prefs(&prefs);
                }
            }
            log::info!("CADCraft startup: persisted settings restored in {} ms", phase.elapsed().as_millis());

            // Keep the custom CAD canvas on its CPU fallback on Android. eframe still uses its
            // normal wgpu renderer for the UI surface; skipping the extra CAD GPU pipeline avoids
            // an additional first-frame shader/buffer initialization path on mobile GPUs.
            log::info!("CADCraft startup: using CPU CAD canvas fallback on Android");
            log::info!("CADCraft startup: app initialization completed in {} ms", startup.elapsed().as_millis());
            Ok(Box::new(AndroidShell(cad)))
        }),
    );

    if let Err(error) = result {
        log::error!("CADCraft Android exited with an error: {error}");
    }
}
