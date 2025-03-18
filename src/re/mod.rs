mod letter;
mod pattern;

use letter::Letters;
use pattern::{parse_pattern, Pattern};

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

        let (patterns, rest) = parse_pattern(expr);

        // TODO:
        // Error handling when rest is not empty
        if !rest.is_empty() {
            panic!("Cannot parse regexp completely!");
        }

        Self {
            start_anchor,
            end_anchor,
            patterns,
        }
    }

    pub fn is_match(&self, s: &str) -> bool {
        let mut cur_pos: usize = 0;
        let mut prev_pat: Option<&Pattern<'a>> = None;

        if !self.start_anchor {
            // Search the first position
            cur_pos = match self.patterns.first().and_then(|p| p.search_match_pos(s)) {
                Some(pos) => pos,
                None => {
                    return false;
                }
            };
        }

        for pat in self.patterns.iter() {
            if cur_pos > s.len() {
                return false;
            }

            if pat.evaluate_with_next() {
                prev_pat = Some(pat);
                continue;
            }

            if let Some(prev) = prev_pat.take() {
                match pat
                    .search_match_pos(&s[cur_pos..])
                    .and_then(|bytes| prev.match_size(&s[cur_pos..(cur_pos + bytes)]))
                {
                    Some(size) => {
                        cur_pos += size;
                    }
                    None => {
                        return false;
                    }
                }
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

        if let Some(pat) = prev_pat.take() {
            match pat.match_size(&s[cur_pos..]) {
                Some(size) => {
                    cur_pos += size;
                }
                None => {
                    return false;
                }
            }
        }

        if self.end_anchor {
            return cur_pos == s.len();
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
}
