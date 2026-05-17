use fluent_bundle::{FluentArgs, FluentBundle, FluentResource, FluentValue};
use unic_langid::LanguageIdentifier;

pub struct I18n {
    bundle: FluentBundle<FluentResource>,
}

impl I18n {
    pub fn new(lang: &str, ftl_content: &str) -> Self {
        let langid: LanguageIdentifier = lang.parse().unwrap_or_else(|_| "ru".parse().unwrap());
        let resource =
            FluentResource::try_new(ftl_content.to_string()).expect("Failed to parse FTL content");
        let mut bundle = FluentBundle::new(vec![langid]);
        bundle
            .add_resource(resource)
            .expect("Failed to add FTL resource");
        Self { bundle }
    }

    pub fn get(&self, key: &str) -> String {
        let msg = match self.bundle.get_message(key) {
            Some(m) => m,
            None => return key.to_string(),
        };
        let pattern = match msg.value() {
            Some(p) => p,
            None => return key.to_string(),
        };
        let mut errors = vec![];
        self.bundle
            .format_pattern(pattern, None, &mut errors)
            .to_string()
    }

    pub fn format(&self, key: &str, args: &[(&str, &str)]) -> String {
        let msg = match self.bundle.get_message(key) {
            Some(m) => m,
            None => return key.to_string(),
        };
        let pattern = match msg.value() {
            Some(p) => p,
            None => return key.to_string(),
        };
        let mut fluent_args = FluentArgs::new();
        for (k, v) in args {
            fluent_args.set(*k, FluentValue::from(*v));
        }
        let mut errors = vec![];
        self.bundle
            .format_pattern(pattern, Some(&fluent_args), &mut errors)
            .to_string()
    }
}
