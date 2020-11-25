
use curl::easy::Easy;
use std::str;

use crate::data;

pub fn run(mut web_struct: data::Sites, list: &[&str]) -> data::Sites {
    for &site in list {
        match web_struct.get(site.to_string()) {
            Some(x) => {
		let mut res = Vec::new();
                let mut easy = Easy::new();
                easy.url(&x).unwrap();
		{
		    let mut transfer = easy.transfer();
                    transfer.write_function(|data| {
			res.extend_from_slice(data);
			Ok(data.len())
                    }).unwrap();
                    transfer.perform().unwrap();
		}
		let data = str::from_utf8(&res).unwrap().to_string();
		web_struct.replace(site.clone().to_string(), data.clone());
            }
            None => (),
        }
    }
    web_struct
}
