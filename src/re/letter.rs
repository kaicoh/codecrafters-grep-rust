#[derive(Debug)]
pub struct Letters<'a> {
    inner: &'a str,
    cursor: usize,
}

impl<'a> Letters<'a> {
    pub fn new(inner: &'a str) -> Self {
        Self { inner, cursor: 0 }
    }

    pub fn tail(&self) -> &'a str {
        &self.inner[self.cursor..]
    }

    fn get(&self, bytes: usize) -> Option<&'a str> {
        self.inner.get(self.cursor..self.cursor + bytes)
    }
}

impl<'a> Iterator for Letters<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.inner.len() {
            return None;
        }

        let mut bytes = 1;
        let mut next_char = self.get(bytes);

        while next_char.is_none() {
            bytes += 1;

            if self.cursor + bytes > self.inner.len() {
                self.cursor = self.inner.len();
                return Some(self.tail());
            }

            next_char = self.get(bytes);
        }

        self.cursor += bytes;
        next_char
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_iterates_over_ascii_characters() {
        let mut l = Letters::new("abc");
        assert_eq!(l.next(), Some("a"));
        assert_eq!(l.next(), Some("b"));
        assert_eq!(l.next(), Some("c"));
        assert_eq!(l.next(), None);

        let mut l = Letters::new("\\w");
        assert_eq!(l.next(), Some("\\"));
        assert_eq!(l.next(), Some("w"));
        assert_eq!(l.next(), None);
    }

    #[test]
    fn it_iterates_over_non_ascii_characters() {
        let mut l = Letters::new("🗻∈🌏");
        assert_eq!(l.next(), Some("🗻"));
        assert_eq!(l.next(), Some("∈"));
        assert_eq!(l.next(), Some("🌏"));
        assert_eq!(l.next(), None);

        let mut l = Letters::new("🗻b🌏");
        assert_eq!(l.next(), Some("🗻"));
        assert_eq!(l.next(), Some("b"));
        assert_eq!(l.next(), Some("🌏"));
        assert_eq!(l.next(), None);
    }
}
