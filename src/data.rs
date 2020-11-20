use std::collections::HashMap;

pub struct Sites {
    pub sites: HashMap<String, String>,
}

impl Sites {
    pub fn get(&mut self, url: String) -> Option<String> {
	self.sites.get_mut(&url).cloned()
    }

    pub fn replace(&mut self, name: String, data: String) -> &mut Sites {
	self.rm(name.clone()).add(name,data)
    }

    pub fn rm(&mut self, name: String) -> &mut Sites {
	self.sites.remove(&name);
	self
    }

    pub fn add(&mut self, name: String, data: String) -> &mut Sites {
	self.sites.insert(name, data);
	self
    }
    
    pub fn init() -> Sites {
        let mut map = HashMap::new();
        map.insert(
	    "komplett".to_string(),
	    "https://www.komplett.se/product/1172141/datorutrustning/datorkomponenter/processor/amd-ryzen-9-5950x-processor".to_string(),
	);
        map.insert(
            "inet".to_string(),
            "https://www.inet.se/produkt/5303477/amd-ryzen-9-5950x-3-4-ghz-72mb".to_string(),
        );
        map.insert(
	    "webhallen".to_string(),
	    "https://www.webhallen.com/se/product/326491-AMD-Ryzen-9-5950X-16-cores-32-threads-4-9-GHz".to_string(),
	);
        map.insert(
	    "proshop".to_string(),
	    "https://www.proshop.se/CPU/AMD-Ryzen-9-5950X-CPU-16-kaernor-34-GHz-AMD-AM4-AMD-Boxed-WOF-utan-kylare/2884175".to_string(),
	);
        map.insert(
	    "elgiganten".to_string(),
	    "https://www.elgiganten.se/product/datorer-tillbehor/datorkomponenter/processor-cpu/228244/amd-ryzen-9-5950x-processor-box".to_string(),
	);

        Sites { sites: map }
    }
}
