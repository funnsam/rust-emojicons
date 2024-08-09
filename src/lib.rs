use std::fmt;
use regex::{
    Regex,
    Captures,
};

/// Newtype used for substituting emoji codes for emoji
///
/// Leaves the notation intact if a corresponding emoji is not found in the
/// lookup table.
pub struct EmojiFormatter<'a>(pub &'a str);

impl<'a> std::fmt::Display for EmojiFormatter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let re = Regex::new(r":([a-zA-Z0-9_\+\-]+):").unwrap();

        let result = re.replace_all(self.0, EmojiReplacer);

        write!(f, "{}", result)
    }
}

struct EmojiReplacer;

impl regex::Replacer for EmojiReplacer {
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
        let sym = caps.get(0).unwrap().into();
        dst.push_str(emojis::get_by_shortcode(sym).map_or(sym, emojis::Emoji::as_str));
    }
}
