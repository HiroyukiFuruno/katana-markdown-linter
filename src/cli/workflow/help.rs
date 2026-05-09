mod en;
mod ja;

use crate::cli::args::HelpTopic;
use crate::i18n::Locale;

pub(super) fn run_help(topic: Option<HelpTopic>, locale: Locale) -> i32 {
    println!("{}", help_text(topic, locale));
    0
}

fn help_text(topic: Option<HelpTopic>, locale: Locale) -> &'static str {
    match locale {
        Locale::Ja => ja::japanese_help_text(topic),
        _ => en::english_help_text(topic),
    }
}
