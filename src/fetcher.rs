extern crate curl;

use std::io::{stdout, Write};
use curl::easy::Easy;


use crate::data;

pub fn run(mut web_struct: data::Sites) -> data::Sites {
    let all = ["inet", "komplett", "webhallen", "proshop", "elgiganten"];
    for &site in &all {
	match web_struct.sites.get(site).cloned() {
	    Some(ref mut x) => {let mut easy = Easy::new();
			easy.url(x).unwrap();
			easy.write_function(|data| {
			    stdout().write_all(data).unwrap();
			    Ok(data.len())
			}).unwrap();
			easy.perform().unwrap();
			web_struct.sites.remove(site);
			web_struct.sites.insert(x.to_string(), easy.response_code().unwrap().to_string());
	    },
	    None => {},
	    
	}
    }
    web_struct
}