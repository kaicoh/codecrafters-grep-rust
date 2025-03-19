mod letter;
mod pattern;

use letter::Letters;
use pattern::{parse_pattern, search_match_size, Pattern};

#[derive(Debug, PartialEq)]
pub struct Regex<'a> {
    start_anchor: bool,
    end_anchor: bool,
    patterns: Vec<Pattern<'a>>,
}

impl<'a> Regex<'a> {
    pub fn new(expr: &'a str) -> Self {
        let start_anchor = expr.starts_with('^');
        let expr = if start_anchor { &expr[1..] } else { expr };

        let end_anchor = expr.ends_with('$');
        let expr = if end_anchor {
            &expr[..expr.len() - 1]
        } else {
            expr
        };

        let parsed = parse_pattern(expr);

        // TODO:
        // Error handling when rest is not empty
        if !parsed.completed() {
            panic!("Cannot parse regexp completely!");
        }

        Self {
            start_anchor,
            end_anchor,
            patterns: parsed.patterns(),
        }
    }

    pub fn is_match(&self, s: &str) -> bool {
        let mut cur_pos: usize = 0;

        if !self.start_anchor {
            // Search the first position
            cur_pos = match self.patterns.first().and_then(|p| p.search_match_pos(s)) {
                Some(pos) => pos,
                None => {
                    return false;
                }
            };
        }
        let s = &s[cur_pos..];
        let matched_pos = match search_match_size(&self.patterns, s) {
            Some(size) => size,
            None => {
                return false;
            }
        };

        if self.end_anchor {
            return matched_pos == s.len();
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_matches_literals() {
        let r = Regex::new("a");
        assert!(r.is_match("abc"));
        assert!(r.is_match("123abc"));
        assert!(!r.is_match("xyz"));
    }

    #[test]
    fn it_matches_digits() {
        let r = Regex::new("\\d");
        assert!(r.is_match("apple123"));
        assert!(!r.is_match("xyz"));
    }

    #[test]
    fn it_matches_alphanumeric_characters() {
        let r = Regex::new("\\w");
        assert!(r.is_match("alpha-num3ric"));
        assert!(!r.is_match("$!?"));
    }

    #[test]
    fn it_matches_wildcard() {
        let r = Regex::new("d.g");
        assert!(r.is_match("dog"));
        assert!(r.is_match("dig"));
        assert!(!r.is_match("cog"));

        let r = Regex::new("g.+");
        assert!(r.is_match("goøö0Ogol"));

        let r = Regex::new("g.+gol");
        assert!(r.is_match("goøö0Ogol"));
    }

    #[test]
    fn it_matches_positive_character_group() {
        let r = Regex::new("[abc]");
        assert!(r.is_match("apple"));
        assert!(!r.is_match("dog"));
    }

    #[test]
    fn it_matches_negative_character_group() {
        let r = Regex::new("[^abc]");
        assert!(r.is_match("dog"));
        assert!(!r.is_match("cab"));
    }

    #[test]
    fn it_matches_combining_character_classes() {
        let r = Regex::new("\\d apple");
        assert!(r.is_match("1 apple"));
        assert!(!r.is_match("1 orange"));

        let r = Regex::new("\\d\\d\\d apple");
        assert!(r.is_match("100 apple"));
        assert!(!r.is_match("1 apple"));

        let r = Regex::new("\\d \\w\\w\\ws");
        assert!(r.is_match("3 dogs"));
        assert!(r.is_match("4 cats"));
        assert!(!r.is_match("1 dog"));
    }

    #[test]
    fn it_matches_with_start_anchor() {
        let r = Regex::new("^log");
        assert!(r.is_match("logs"));
        assert!(!r.is_match("slog"));
    }

    #[test]
    fn it_matches_with_end_anchor() {
        let r = Regex::new("dog$");
        assert!(r.is_match("dog"));
        assert!(!r.is_match("dogs"));
    }

    #[test]
    fn it_matches_zero_or_one_times() {
        let r = Regex::new("dogs?");
        assert!(r.is_match("dog"));
        assert!(r.is_match("dogs"));
        assert!(!r.is_match("cat"));
    }

    #[test]
    fn it_matches_alternation() {
        let r = Regex::new("(dog|cat)");
        assert!(r.is_match("dog"));
        assert!(r.is_match("cat"));
        assert!(!r.is_match("dig"));
    }
}
