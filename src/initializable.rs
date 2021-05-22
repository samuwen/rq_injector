use crate::locales::Locale;

pub trait Initializable {
    fn init_text(&self, locale: &Locale);
}
