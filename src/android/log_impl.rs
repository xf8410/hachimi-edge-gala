use android_logger::FilterBuilder;
use simplelog::{ConfigBuilder, LevelFilter, WriteLogger};
use std::fs::File;

pub fn init(filter_level: log::LevelFilter, file_logging: bool) {
    if file_logging {
        let mut path = super::utils::get_game_dir();
        path.push("hachimi.log");

        // First run has no hachimi dir yet; File::create alone silently fails.
        if let Some(parent) = path.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                eprintln!("Failed to create log dir {}: {}", parent.display(), e);
            }
        }

        if let Ok(file) = File::create(path) {
            let config = ConfigBuilder::new()
                .set_target_level(LevelFilter::Error)
                .add_filter_ignore_str("sqlparser")
                .set_time_format_rfc3339()
                .build();

            if WriteLogger::init(filter_level, config, file).is_ok() {
                return;
            }
        }
    }

    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(filter_level)
            .with_filter(
                FilterBuilder::new()
                    .filter_level(filter_level)
                    .filter_module("sqlparser", log::LevelFilter::Off) // annoying
                    .build()
            )
            .with_tag("Hachimi")
    );
}