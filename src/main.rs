use fern::colors::{Color, ColoredLevelConfig};
use log::error;
use structopt::StructOpt;

use ryzen::data;
use ryzen::fetcher;

#[derive(StructOpt, Debug, Clone)]
#[structopt(
    name = "ryzen",
    setting = structopt::clap::AppSettings::ColoredHelp,
    about = " "
)]

struct Ryzen {
    #[structopt(short = "v", long = "verbose", help = "Turn on verbosity")]
    verbose: bool,

    #[structopt(subcommand)]
    cmd: Goal,
}

impl Ryzen {
    fn run(self) {
        self.logging();
        match self.cmd.run() {
            Ok(()) => (),
            Err(err) => error!("{:?}", &err),
        }
    }

    fn logging(&self) {
        let colors_line = ColoredLevelConfig::new()
            .error(Color::Red)
            .warn(Color::Yellow)
            .info(Color::White)
            .debug(Color::BrightBlack)
            .trace(Color::BrightBlack);
        let colors_level = colors_line.clone().info(Color::Green);
        fern::Dispatch::new()
            .format(move |out, message, record| {
                out.finish(format_args!(
                    "{color_line}{date} {level}{color_line} :: {message}\x1B[0m",
                    color_line = format_args!(
                        "\x1B[{}m",
                        colors_line.get_color(&record.level()).to_fg_str()
                    ),
                    date = chrono::Local::now().format("%H:%M:%S"),
                    level = colors_level.color(record.level()),
                    message = message,
                ));
            })
            .level(match self.verbose {
                true => log::LevelFilter::Debug,
                false => log::LevelFilter::Info,
            })
            .level_for("pretty_colored", log::LevelFilter::Trace)
            .chain(std::io::stderr())
            .apply()
            .unwrap();
    }
}

#[derive(StructOpt, Debug, Clone)]
enum Goal {
    Checkall(Choice),
}

impl Goal {
    fn run(self) -> Result<(), anyhow::Error> {
        match self {
            Goal::Checkall(choice) => choice.checkall(),
        }
    }
}

#[derive(StructOpt, Debug, Clone)]
#[structopt(
    name = "checkall",
    about = "Check all the available websites for availability",
    help = "Takes no other input, just checks all the sites."
)]

struct Choice {}

impl Choice {
    fn checkall(self) -> Result<(), anyhow::Error> {
        let mut web_struct = data::Sites::init();
        let mut sites = fetcher::run(web_struct);
        println!("{:?}", sites.sites.get("inet"));
        println!("{:?}", sites.sites.get("komplett"));
        Ok(())
    }
}

fn main() {
    Ryzen::from_args().run();
}
