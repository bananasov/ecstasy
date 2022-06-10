use crate::error::EcstasyError;
use log::{debug, info};
use fern::colors::{Color, ColoredLevelConfig};

pub struct EcstasyLogger;

impl EcstasyLogger {
    pub fn init(verbose: bool) -> Result<(), EcstasyError> {
        let colors_line = ColoredLevelConfig::new()
            .error(Color::Red)
            .warn(Color::Yellow)
            .info(Color::BrightBlue)
            .debug(Color::White)
            .trace(Color::BrightBlack);

        let log_level = if verbose {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        };
        fern::Dispatch::new()
            .format(move |out, message, record| {
                out.finish(format_args!(
                    "[{color_line}{level}\x1B[0m | {date} | {target}] {message}",
                    level = record.level(),
                    color_line = format_args!(
                        "\x1B[{}m",
                        colors_line.get_color(&record.level()).to_fg_str()
                    ),
                    date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S.%3f"),
                    target = record.target(),
                    message = message
                ))
            })
            .level(log_level)
            .chain(std::io::stdout())
            .apply()?;
        info!("Logger initialized!");
        debug!("Logging level set to debug verbosity.");
        Ok(())
    }
}
