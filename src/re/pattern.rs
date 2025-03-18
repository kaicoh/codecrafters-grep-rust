use super::Letters;

#[derive(Debug, PartialEq)]
pub enum Pattern<'a> {
    Lit(&'a str),
    AlphaNumeric,
    Digit,
    Wildcard,
    PGroup(Vec<Pattern<'a>>),
    NGroup(Vec<Pattern<'a>>),
    MoreThanZero(Box<Pattern<'a>>),
    MoreThanOne(Box<Pattern<'a>>),
    ZeroOrOne(Box<Pattern<'a>>),
}

impl Pattern<'_> {
    pub fn search_match_pos(&self, s: &str) -> Option<usize> {
        let mut pos = 0;
        let mut letters = Letters::new(s);

        while self.match_size(letters.tail()).is_none() {
            let l = letters.next()?;
            pos += l.len();

            if pos >= s.len() {
                return None;
            }
        }
        Some(pos)
    }

    pub fn evaluate_with_next(&self) -> bool {
        matches!(
            self,
            Self::MoreThanOne(_) | Self::MoreThanZero(_) | Self::ZeroOrOne(_)
        )
    }

    pub fn match_size(&self, s: &str) -> Option<usize> {
        let mut letters = Letters::new(s);

        match self {
            Self::Lit(lit) => letters
                .next()
                .and_then(|l| if l == *lit { Some(l.len()) } else { None }),
            Self::AlphaNumeric => letters.next().and_then(|l| {
                if is_ascii_alphanumeric(l) {
                    Some(l.len())
                } else {
                    None
                }
            }),
            Self::Digit => letters.next().and_then(|l| {
                if is_ascii_digit(l) {
                    Some(l.len())
                } else {
                    None
                }
            }),
            Self::Wildcard => letters.next().map(|l| l.len()),
            Self::PGroup(pats) => pats.iter().filter_map(|pat| pat.match_size(s)).next(),
            Self::NGroup(pats) => {
                if pats.iter().all(|pat| pat.match_size(s).is_none()) {
                    // FIXME:
                    // This is wroing.
                    Some(1)
                } else {
                    None
                }
            }
            Self::MoreThanZero(pat) => {
                let mut acc = 0;

                while let Some(size) = pat.match_size(&s[acc..]) {
                    acc += size;
                }

                Some(acc)
            }
            Self::MoreThanOne(pat) => {
                let mut acc = 0;

                if let Some(size) = pat.match_size(s) {
                    acc += size;
                } else {
                    return None;
                }

                while let Some(size) = pat.match_size(&s[acc..]) {
                    acc += size;
                }

                Some(acc)
            }
            Self::ZeroOrOne(pat) => {
                let size = pat.match_size(s).unwrap_or(0);
                Some(size)
            }
        }
    }
}

#[derive(Debug)]
enum PatternChar<'a> {
    Itself(Pattern<'a>),
    MoreThanZero,
    MoreThanOne,
    ZeroOrOne,
    PGroupOpen,
    NGroupOpen,
    GroupClose,
}

impl<'a> PatternChar<'a> {
    fn pick(expr: &'a str) -> Option<(Self, &'a str)> {
        let mut letters = Letters::new(expr);

        match letters.next()? {
            "\\" => match letters.next()? {
                "w" => {
                    let pat = Pattern::AlphaNumeric;
                    Some((PatternChar::Itself(pat), letters.tail()))
                }
                "d" => {
                    let pat = Pattern::Digit;
                    Some((PatternChar::Itself(pat), letters.tail()))
                }
                l => {
                    let pat = Pattern::Lit(l);
                    Some((PatternChar::Itself(pat), letters.tail()))
                }
            },
            "." => {
                let pat = Pattern::Wildcard;
                Some((PatternChar::Itself(pat), letters.tail()))
            }
            "[" => {
                if letters.tail().starts_with('^') {
                    letters.next();
                    Some((PatternChar::NGroupOpen, letters.tail()))
                } else {
                    Some((PatternChar::PGroupOpen, letters.tail()))
                }
            }
            "]" => Some((PatternChar::GroupClose, letters.tail())),
            "+" => Some((PatternChar::MoreThanOne, letters.tail())),
            "*" => Some((PatternChar::MoreThanZero, letters.tail())),
            "?" => Some((PatternChar::ZeroOrOne, letters.tail())),
            l => {
                let pat = Pattern::Lit(l);
                Some((PatternChar::Itself(pat), letters.tail()))
            }
        }
    }
}

pub fn parse_pattern<'a>(expr: &'a str) -> (Vec<Pattern<'a>>, &'a str) {
    let mut rest_expr = expr;
    let mut patterns: Vec<Pattern<'a>> = vec![];

    while let Some((chr, mut rest)) = PatternChar::pick(rest_expr) {
        match chr {
            PatternChar::Itself(p) => {
                patterns.push(p);
            }
            PatternChar::MoreThanZero => {
                // TODO:
                // handle when pop method returns None
                if let Some(p) = patterns.pop() {
                    patterns.push(Pattern::MoreThanZero(Box::new(p)));
                }
            }
            PatternChar::MoreThanOne => {
                // TODO:
                // handle when pop method returns None
                if let Some(p) = patterns.pop() {
                    patterns.push(Pattern::MoreThanOne(Box::new(p)));
                }
            }
            PatternChar::ZeroOrOne => {
                // TODO:
                // handle when pop method returns None
                if let Some(p) = patterns.pop() {
                    patterns.push(Pattern::ZeroOrOne(Box::new(p)));
                }
            }
            PatternChar::PGroupOpen => {
                let (inner, remaining) = parse_pattern(rest);
                patterns.push(Pattern::PGroup(inner));
                rest = remaining;
            }
            PatternChar::NGroupOpen => {
                let (inner, remaining) = parse_pattern(rest);
                patterns.push(Pattern::NGroup(inner));
                rest = remaining;
            }
            PatternChar::GroupClose => {
                return (patterns, rest);
            }
        }

        rest_expr = rest;
    }

    (patterns, rest_expr)
}

fn is_ascii_alphanumeric(s: &str) -> bool {
    is_ascii_alphabet(s) || is_ascii_digit(s) || s == "_"
}

fn is_ascii_alphabet(s: &str) -> bool {
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".contains(s)
}

fn is_ascii_digit(s: &str) -> bool {
    "0123456789".contains(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_lit() {
        let expr = "a";
        let (patterns, rest) = parse_pattern(expr);
        assert_eq!(patterns, vec![Pattern::Lit("a")]);
        assert_eq!(rest, "");
    }

    #[test]
    fn it_parses_alphanumeric() {
        let expr = "\\w";
        let (patterns, rest) = parse_pattern(expr);
        assert_eq!(patterns, vec![Pattern::AlphaNumeric]);
        assert_eq!(rest, "");
    }

    #[test]
    fn it_parses_digit() {
        let expr = "\\d";
        let (patterns, rest) = parse_pattern(expr);
        assert_eq!(patterns, vec![Pattern::Digit]);
        assert_eq!(rest, "");
    }

    #[test]
    fn it_parses_wildcard() {
        let expr = ".";
        let (patterns, rest) = parse_pattern(expr);
        assert_eq!(patterns, vec![Pattern::Wildcard]);
        assert_eq!(rest, "");
    }

    #[test]
    fn it_parses_positive_group() {
        let expr = "[abc]";
        let (patterns, rest) = parse_pattern(expr);
        let expected = vec![Pattern::PGroup(vec![
            Pattern::Lit("a"),
            Pattern::Lit("b"),
            Pattern::Lit("c"),
        ])];
        assert_eq!(patterns, expected);
        assert_eq!(rest, "");
    }

    #[test]
    fn it_parses_negative_group() {
        let expr = "[^xyz]";
        let (patterns, rest) = parse_pattern(expr);
        let expected = vec![Pattern::NGroup(vec![
            Pattern::Lit("x"),
            Pattern::Lit("y"),
            Pattern::Lit("z"),
        ])];
        assert_eq!(patterns, expected);
        assert_eq!(rest, "");
    }

    #[test]
    fn it_parses_more_than_one_pattern() {
        let expr = "\\w+";
        let (patterns, rest) = parse_pattern(expr);
        let expected = vec![Pattern::MoreThanOne(Box::new(Pattern::AlphaNumeric))];
        assert_eq!(patterns, expected);
        assert_eq!(rest, "");

        let expr = "[abc]+";
        let (patterns, rest) = parse_pattern(expr);
        let expected = vec![Pattern::MoreThanOne(Box::new(Pattern::PGroup(vec![
            Pattern::Lit("a"),
            Pattern::Lit("b"),
            Pattern::Lit("c"),
        ])))];
        assert_eq!(patterns, expected);
        assert_eq!(rest, "");
    }

    #[test]
    fn it_parses_more_than_zero_pattern() {
        let expr = "\\w*";
        let (patterns, rest) = parse_pattern(expr);
        let expected = vec![Pattern::MoreThanZero(Box::new(Pattern::AlphaNumeric))];
        assert_eq!(patterns, expected);
        assert_eq!(rest, "");

        let expr = "[abc]*";
        let (patterns, rest) = parse_pattern(expr);
        let expected = vec![Pattern::MoreThanZero(Box::new(Pattern::PGroup(vec![
            Pattern::Lit("a"),
            Pattern::Lit("b"),
            Pattern::Lit("c"),
        ])))];
        assert_eq!(patterns, expected);
        assert_eq!(rest, "");
    }

    #[test]
    fn it_parses_zero_or_one_pattern() {
        let expr = "\\w?";
        let (patterns, rest) = parse_pattern(expr);
        let expected = vec![Pattern::ZeroOrOne(Box::new(Pattern::AlphaNumeric))];
        assert_eq!(patterns, expected);
        assert_eq!(rest, "");

        let expr = "[abc]?";
        let (patterns, rest) = parse_pattern(expr);
        let expected = vec![Pattern::ZeroOrOne(Box::new(Pattern::PGroup(vec![
            Pattern::Lit("a"),
            Pattern::Lit("b"),
            Pattern::Lit("c"),
        ])))];
        assert_eq!(patterns, expected);
        assert_eq!(rest, "");
    }

    #[test]
    fn it_parses_nested_group() {
        let expr = "[a[bc]]";
        let (patterns, rest) = parse_pattern(expr);
        let expected = vec![Pattern::PGroup(vec![
            Pattern::Lit("a"),
            Pattern::PGroup(vec![Pattern::Lit("b"), Pattern::Lit("c")]),
        ])];
        assert_eq!(patterns, expected);
        assert_eq!(rest, "");

        let expr = "[a[^bc]]";
        let (patterns, rest) = parse_pattern(expr);
        let expected = vec![Pattern::PGroup(vec![
            Pattern::Lit("a"),
            Pattern::NGroup(vec![Pattern::Lit("b"), Pattern::Lit("c")]),
        ])];
        assert_eq!(patterns, expected);
        assert_eq!(rest, "");
    }

    #[test]
    fn it_parses_multiple_patterns() {
        let expr = "\\d apple";
        let (patterns, rest) = parse_pattern(expr);
        let expected = vec![
            Pattern::Digit,
            Pattern::Lit(" "),
            Pattern::Lit("a"),
            Pattern::Lit("p"),
            Pattern::Lit("p"),
            Pattern::Lit("l"),
            Pattern::Lit("e"),
        ];
        assert_eq!(patterns, expected);
        assert_eq!(rest, "");
    }
}
