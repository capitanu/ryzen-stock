use fern::colors::{Color, ColoredLevelConfig};
use log::error;
use structopt::StructOpt;
use std::collections::HashMap;

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
    All(Choice),
    Komplett(Choice),
    Inet(Choice),
    Proshop(Choice),
}

impl Goal {
    fn run(self) -> Result<(), anyhow::Error> {
        match self {
            Goal::All(choice) => choice.check(&["komplett","inet","proshop"]),
	    Goal::Inet(choice) => choice.check(&["inet"]),
	    Goal::Komplett(choice) => choice.check(&["komplett"]),
	    Goal::Proshop(choice) => choice.check(&["proshop"]),
	    
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
    fn parse(self, arr: &[&str], map: &mut HashMap<String,String>){
	let mut fetched;
	for &shop in arr {
	    fetched = map.get(shop).cloned();
	    match fetched {
		Some(mut x) => {
		    if shop == "inet" {
			let index = x.find("I lager <!-- -->");
			match index {
			    Some(integer) => {
				let mut status = x.split_off(integer+16);
				let _useless = status.split_off(14);
				println!("Inet: {}", status.clone());
				map.remove(shop);
				map.insert(shop.to_string(), status);
				()
			    },
			    None => (),
			}
		    };
		    if shop == "proshop" {
			let index = x.find("<div class=\"site-stock-text site-inline\">");
			match index {
			    Some(integer) => {
				let mut status = x.split_off(integer+90);
				let _useless = status.split_off(11);
				println!("Proshop: {}", status.clone());
				map.remove(shop);
				map.insert(shop.to_string(), status);
				()
			    },
			    None => (),
			}
		    };
		    if shop == "komplett" {
			let index = x.find("<span class=\"stockstatus-stock-details\">");
			match index {
			    Some(integer) => {
				let mut status = x.split_off(integer+92);
				let _useless = status.split_off(11);
				println!("Komplett: {}", status.clone());
				map.remove(shop);
				map.insert(shop.to_string(), status);
				()
			    },
			    None => (),
			}
		    };
		    ()
		},
		None => (),
	    }
	}
    }
    fn check(self, mut arr: &[&str]) -> Result<(), anyhow::Error> {
        let web_struct = data::Sites::init();
        let mut sites = fetcher::run(web_struct, arr);
	let mut fetched;
	for &shop in arr {
	    fetched = sites.sites.get(shop).cloned();
	    match fetched {
		Some(x) => {
		    sites.sites.remove(shop);
		    sites.sites.insert(shop.to_string(), x.to_string());
		    ()	    
		},
		None => (),
	    }
	}
	self.parse(&mut arr, &mut sites.sites);
        //println!("{:?}", sites.sites.get("proshop"));
        Ok(())
    }
}

fn main() {
    Ryzen::from_args().run();
}
