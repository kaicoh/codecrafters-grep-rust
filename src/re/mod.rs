mod pattern;

use pattern::{parse_pattern, Pattern};

#[derive(Debug, PartialEq)]
pub struct Regex {
    patterns: Vec<Pattern>,
}

impl Regex {
    pub fn new(expr: &str) -> Self {
        let (patterns, rest) = parse_pattern(expr);

        // TODO:
        // Error handling when rest is not empty
        if !rest.is_empty() {
            panic!("Cannot parse regexp completely!");
        }

        Self { patterns }
    }

    pub fn is_match(&self, s: &str) -> bool {
        let mut cur_pos: usize = 0;

        if let Some(pat) = self.patterns.first() {
            while pat.match_size(&s[cur_pos..]).is_none() {
                cur_pos += 1;

                if cur_pos >= s.len() {
                    return false;
                }
            }
        } else {
            return false;
        }

        for pat in self.patterns.iter() {
            if cur_pos >= s.len() {
                return false;
            }

            match pat.match_size(&s[cur_pos..]) {
                Some(size) => {
                    cur_pos += size;
                }
                None => {
                    return false;
                }
            }
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
}
