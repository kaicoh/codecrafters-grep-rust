#[derive(Debug, PartialEq)]
pub enum Pattern {
    Lit(char),
    AlphaNumeric,
    Digit,
    PGroup(Vec<Pattern>),
    NGroup(Vec<Pattern>),
    MoreThanZero(Box<Pattern>),
    MoreThanOne(Box<Pattern>),
    ZeroOrOne(Box<Pattern>),
}

impl Pattern {
    fn maybe(expr: &str) -> Option<(MaybePattern, &str)> {
        let mut chars = expr.chars();

        match chars.next()? {
            '\\' => match chars.next()? {
                'w' => {
                    let pat = Pattern::AlphaNumeric;
                    Some((MaybePattern::Itself(pat), &expr[2..]))
                }
                'd' => {
                    let pat = Pattern::Digit;
                    Some((MaybePattern::Itself(pat), &expr[2..]))
                }
                '\\' => {
                    let pat = Pattern::Lit('\\');
                    Some((MaybePattern::Itself(pat), &expr[2..]))
                }
                c => {
                    unimplemented!("Unknown pattern: \\{c}")
                }
            },
            '[' => match chars.next()? {
                '^' => Some((MaybePattern::NGroupOpen, &expr[2..])),
                _ => Some((MaybePattern::PGroupOpen, &expr[1..])),
            },
            ']' => Some((MaybePattern::GroupClose, &expr[1..])),
            '+' => Some((MaybePattern::MoreThanOne, &expr[1..])),
            '*' => Some((MaybePattern::MoreThanZero, &expr[1..])),
            '?' => Some((MaybePattern::ZeroOrOne, &expr[1..])),
            c => {
                let pat = Pattern::Lit(c);
                Some((MaybePattern::Itself(pat), &expr[1..]))
            }
        }
    }

    pub fn match_size(&self, s: &str) -> Option<usize> {
        let mut chars = s.chars();
        match self {
            Self::Lit(c) => chars
                .next()
                .and_then(|ch| if ch == *c { Some(1) } else { None }),
            Self::AlphaNumeric => chars.next().and_then(|ch| {
                if ch.is_ascii_alphanumeric() || ch == '_' {
                    Some(1)
                } else {
                    None
                }
            }),
            Self::Digit => chars
                .next()
                .and_then(|ch| if ch.is_ascii_digit() { Some(1) } else { None }),
            Self::PGroup(pats) => pats.iter().filter_map(|pat| pat.match_size(s)).next(),
            Self::NGroup(pats) => {
                if pats.iter().all(|pat| pat.match_size(s).is_none()) {
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
enum MaybePattern {
    Itself(Pattern),
    MoreThanZero,
    MoreThanOne,
    ZeroOrOne,
    PGroupOpen,
    NGroupOpen,
    GroupClose,
}

pub fn parse_pattern(expr: &str) -> (Vec<Pattern>, &str) {
    let mut rest_expr = expr;
    let mut patterns: Vec<Pattern> = vec![];

    while let Some((pat, mut rest)) = Pattern::maybe(rest_expr) {
        match pat {
            MaybePattern::Itself(p) => {
                patterns.push(p);
            }
            MaybePattern::MoreThanZero => {
                // TODO:
                // handle when pop method returns None
                if let Some(p) = patterns.pop() {
                    patterns.push(Pattern::MoreThanZero(Box::new(p)));
                }
            }
            MaybePattern::MoreThanOne => {
                // TODO:
                // handle when pop method returns None
                if let Some(p) = patterns.pop() {
                    patterns.push(Pattern::MoreThanOne(Box::new(p)));
                }
            }
            MaybePattern::ZeroOrOne => {
                // TODO:
                // handle when pop method returns None
                if let Some(p) = patterns.pop() {
                    patterns.push(Pattern::ZeroOrOne(Box::new(p)));
                }
            }
            MaybePattern::PGroupOpen => {
                let (inner, remaining) = parse_pattern(rest);
                patterns.push(Pattern::PGroup(inner));
                rest = remaining;
            }
            MaybePattern::NGroupOpen => {
                let (inner, remaining) = parse_pattern(rest);
                patterns.push(Pattern::NGroup(inner));
                rest = remaining;
            }
            MaybePattern::GroupClose => {
                return (patterns, rest);
            }
        }

        rest_expr = rest;
    }

    (patterns, rest_expr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_lit() {
        let expr = "a";
        let (patterns, rest) = parse_pattern(expr);
        assert_eq!(patterns, vec![Pattern::Lit('a')]);
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
    fn it_parses_positive_group() {
        let expr = "[abc]";
        let (patterns, rest) = parse_pattern(expr);
        let expected = vec![Pattern::PGroup(vec![
            Pattern::Lit('a'),
            Pattern::Lit('b'),
            Pattern::Lit('c'),
        ])];
        assert_eq!(patterns, expected);
        assert_eq!(rest, "");
    }

    #[test]
    fn it_parses_negative_group() {
        let expr = "[^xyz]";
        let (patterns, rest) = parse_pattern(expr);
        let expected = vec![Pattern::NGroup(vec![
            Pattern::Lit('x'),
            Pattern::Lit('y'),
            Pattern::Lit('z'),
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
            Pattern::Lit('a'),
            Pattern::Lit('b'),
            Pattern::Lit('c'),
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
            Pattern::Lit('a'),
            Pattern::Lit('b'),
            Pattern::Lit('c'),
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
            Pattern::Lit('a'),
            Pattern::Lit('b'),
            Pattern::Lit('c'),
        ])))];
        assert_eq!(patterns, expected);
        assert_eq!(rest, "");
    }

    #[test]
    fn it_parses_nested_group() {
        let expr = "[a[bc]]";
        let (patterns, rest) = parse_pattern(expr);
        let expected = vec![Pattern::PGroup(vec![
            Pattern::Lit('a'),
            Pattern::PGroup(vec![Pattern::Lit('b'), Pattern::Lit('c')]),
        ])];
        assert_eq!(patterns, expected);
        assert_eq!(rest, "");

        let expr = "[a[^bc]]";
        let (patterns, rest) = parse_pattern(expr);
        let expected = vec![Pattern::PGroup(vec![
            Pattern::Lit('a'),
            Pattern::NGroup(vec![Pattern::Lit('b'), Pattern::Lit('c')]),
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
            Pattern::Lit(' '),
            Pattern::Lit('a'),
            Pattern::Lit('p'),
            Pattern::Lit('p'),
            Pattern::Lit('l'),
            Pattern::Lit('e'),
        ];
        assert_eq!(patterns, expected);
        assert_eq!(rest, "");
    }
}
