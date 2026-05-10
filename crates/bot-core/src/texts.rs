use std::collections::HashMap;

pub struct Texts {
    map: HashMap<String, String>,
}

impl Texts {
    pub fn new(map: HashMap<String, String>) -> Self {
        Self { map }
    }

    pub fn get<'a>(&'a self, key: &'a str) -> &'a str {
        self.map.get(key).map(|s| s.as_str()).unwrap_or(key)
    }

    pub fn format(&self, key: &str, replacements: &[(&str, &str)]) -> String {
        let mut text = self.get(key).to_string();
        for (placeholder, value) in replacements {
            text = text.replace(&format!("{{{}}}", placeholder), value);
        }
        text = text.replace("\\n", "\n");
        text
    }
}