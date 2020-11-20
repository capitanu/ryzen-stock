extern crate curl;

use curl::easy::Easy;
use std::io::{stdout, Write};
use std::str;

use crate::data;

pub fn run(mut web_struct: data::Sites) -> data::Sites {
    let all = ["inet", "komplett", "webhallen", "proshop", "elgiganten"];
    for &site in &all {
        match web_struct.sites.get_mut(site).cloned() {
            Some(ref mut x) => {
                let mut easy = Easy::new();
                easy.url(x).unwrap();
                easy.write_function(move |data| {
                    web_struct.sites.remove(site);
                    web_struct
                        .sites
                        .insert(x.to_string(), str::from_utf8(data).unwrap().to_string());
                    Ok(data.len())
                })
                .unwrap();
                easy.perform().unwrap();
            }
            None => (),
        }
    }
    web_struct
}
