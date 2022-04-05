//! Internationalization support.

use std::borrow::Cow;
use std::fmt;

use fluent_bundle::{FluentArgs, FluentResource, FluentValue};
use unic_langid::{langid, LanguageIdentifier};

use self::loader::{Loader, SimpleLoader};

mod loader;

pub const EN_US: LanguageIdentifier = langid!("en-US");

crate::simple_loader!(
    create_loader, "./locales/", "en-US",
    core: "./locales/core.ftl",
    customizer: add_bundle_functions);

fn add_bundle_functions(_bundle: &mut loader::FluentBundle<&'static FluentResource>) {}

pub struct LangArgs<'args> {
    args: FluentArgs<'args>,
}

impl<'args> LangArgs<'args> {
    pub fn v<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<Cow<'args, str>>,
        V: Into<FluentValue<'args>>,
    {
        self.args.set(key, value);
        self
    }
}

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

    pub fn a(&self) -> LangArgs<'_> {
        LangArgs {
            args: FluentArgs::new(),
        }
    }

    pub fn ma(&self, text_id: &str, args: LangArgs<'_>) -> String {
        self.loader.lookup(&self.lang, text_id, Some(&args.args))
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
