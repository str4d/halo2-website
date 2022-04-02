//! Internationalization support.

use std::fmt;

use fluent_bundle::FluentResource;
use unic_langid::{langid, LanguageIdentifier};

use self::loader::{Loader, SimpleLoader};

mod loader;

pub const EN_US: LanguageIdentifier = langid!("en-US");

crate::simple_loader!(
    create_loader, "./locales/", "en-US",
    core: "./locales/core.ftl",
    customizer: add_bundle_functions);

fn add_bundle_functions(_bundle: &mut loader::FluentBundle<&'static FluentResource>) {}

pub struct Language {
    lang: LanguageIdentifier,
    loader: SimpleLoader,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.lang)
    }
}

impl Language {
    pub fn new(lang: LanguageIdentifier) -> Self {
        Language {
            lang,
            loader: create_loader(),
        }
    }

    pub fn m(&self, text_id: &str) -> String {
        self.loader.lookup(&self.lang, text_id, None)
    }
}

/// A locale for which we have translations.
pub struct LocaleInfo {
    pub lang: &'static str,
    pub text: &'static str,
}

/// The list of locales for which we have translations.
///
/// This list is used to create crosslinks between localized versions of pages.
pub const EXPLICIT_LOCALE_INFO: &[LocaleInfo] = &[LocaleInfo {
    lang: "en-US",
    text: "English",
}];
