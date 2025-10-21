use crate::interpolate::*;
mod interpolate;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Match {
    start: usize,
    end: usize,
}

impl Match {
    #[inline]
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    #[inline]
    pub fn start(&self) -> usize {
        self.start
    }

    #[inline]
    pub fn end(&self) -> usize {
        self.end
    }

    #[inline]
    pub fn zero(offset: usize) -> Self {
        Self { start: offset, end: offset }
    }

    #[inline]
    pub fn with_start(&self, start: usize) -> Match {
        assert!(start <= self.end, "{} is not <= {}", start, self.end);
        Self { start, ..*self }
    }

    #[inline]
    pub fn with_end(&self, end: usize) -> Match {
        assert!(end >= self.start, "{} is not >= {}", end, self.start);
        Self { end, ..*self }
    }

    #[inline]
    pub fn offset(&self, amount: usize) -> Match {
        Match {
            start: self.start.checked_add(amount).unwrap(),
            end: self.end.checked_add(amount).unwrap(),
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl std::ops::Index<Match> for [u8] {
    type Output = [u8];

    #[inline]
    fn index(&self, index: Match) -> &[u8] {
        &self[index.start..index.end]
    }
}

impl std::ops::IndexMut<Match> for [u8] {
    #[inline]
    fn index_mut(&mut self, index: Match) -> &mut [u8] {
        &mut self[index.start..index.end]
    }
}

impl std::ops::Index<Match> for str {
    type Output = str;

    #[inline]
    fn index(&self, index: Match) -> &str {
        &self[index.start..index.end]
    }
}

pub trait Captures {
    fn len(&self) -> usize;

    fn get(&self, i: usize) -> Option<Match>;

    fn as_match(&self) -> Match {
        self.get(0).unwrap()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn interpolate<F>(
        &self,
        name_to_index: F,
        haystack: &[u8],
        replacement: &[u8],
        dst: &mut [u8],
    ) where
        F: FnMut(&str) -> Option<usize>,
    {
        todo!()
    }
}

pub trait Matcher {
    type Captures: Captures;
    type Error: std::fmt::Display;

    fn find_at(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> Result<Option<Match>, Self::Error>;

    fn new_captures(&self) -> Result<Self::Captures, Self::Error>;

    fn capture_count(&self) -> usize {
        0
    }

    fn capture_index(&self, _name: &str) -> Option<usize> {
        None
    }

    fn find(&self, haystack: &[u8]) -> Result<Option<Match>, Self::Error> {
        self.find_at(haystack, 0)
    }

    fn find_iter<F>(
        &self,
        haystack: &[u8],
        matched: F,
    ) -> Result<(), Self::Error>
    where
        F: FnMut(Match) -> bool,
    {
        self.find_iter_at(haystack, 0, matched)
    }

    fn find_iter_at<F>(
        &self,
        haystack: &[u8],
        at: usize,
        mut matched: F,
    ) -> Result<(), Self::Error>
    where
        F: FnMut(Match) -> bool,
    {
        self.try_find_iter_at(haystack, at, |m| Ok(matched(m)))
            .map(|r: Result<(), ()>| r.unwrap())
    }

    fn try_find_iter<F, E>(
        &self,
        haystack: &[u8],
        matched: F,
    ) -> Result<Result<(), E>, Self::Error>
    where
        F: FnMut(Match) -> Result<bool, E>,
    {
        self.try_find_iter_at(haystack, 0, matched)
    }

    fn try_find_iter_at<F, E>(
        &self,
        haystack: &[u8],
        at: usize,
        mut matched: F,
    ) -> Result<Result<(), E>, Self::Error>
    where
        F: FnMut(Match) -> Result<bool, E>,
    {
        let mut last_end = at;
        let mut last_match = None;

        loop {
            if last_end > haystack.len() {
                return Ok(Ok(()));
            }
            let m = match self.find_at(haystack, last_end)? {
                None => return Ok(Ok(())),
                Some(m) => m,
            };
            if m.start == m.end {
                last_end = m.end + 1;
                if Some(m.end) == last_match {
                    continue;
                }
            } else {
                last_end = m.end;
            }

            last_match = Some(m.end);
            match matched(m) {
                Ok(true) => return Ok(Ok(())),
                Ok(false) => continue,
                Err(e) => return Ok(Err(e)),
            }
        }
    }

    fn captures_at(
        &self,
        _haystack: &[u8],
        _at: usize,
        _caps: &mut Self::Captures,
    ) -> Result<bool, Self::Error> {
        Ok(false)
    }

    fn captures_iter<F>(
        &self,
        haystack: &[u8],
        caps: &mut Self::Captures,
        matched: F,
    ) -> Result<(), Self::Error>
    where
        F: FnMut(&Self::Captures) -> bool,
    {
        self.captures_iter_at(haystack, 0, caps, matched)
    }

    fn captures_iter_at<F>(
        &self,
        haystack: &[u8],
        at: usize,
        caps: &mut Self::Captures,
        mut matched: F,
    ) -> Result<(), Self::Error>
    where
        F: FnMut(&Self::Captures) -> bool,
    {
        self.try_captures_iter_at(haystack, at, caps, |caps| Ok(matched(caps)))
            .map(|r: Result<(), ()>| r.unwrap())
    }

    fn try_captures_iter<F, E>(
        &self,
        haystack: &[u8],
        caps: &mut Self::Captures,
        matched: F,
    ) -> Result<Result<(), E>, Self::Error>
    where
        F: FnMut(&Self::Captures) -> Result<bool, E>,
    {
        self.try_captures_iter_at(haystack, 0, caps, matched)
    }

    fn try_captures_iter_at<F, E>(
        &self,
        haystack: &[u8],
        at: usize,
        caps: &mut Self::Captures,
        mut matched: F,
    ) -> Result<Result<(), E>, Self::Error>
    where
        F: FnMut(&Self::Captures) -> Result<bool, E>,
    {
        let mut last_end = at;
        let mut last_match = None;

        loop {
            if last_end > haystack.len() {
                return Ok(Ok(()));
            }
            if !self.captures_at(haystack, last_end, caps)? {
                return Ok(Ok(()));
            }
            let m = caps.get(0).unwrap();
            if m.start == m.end {
                last_end = m.end + 1;
                if Some(m.end) == last_match {
                    continue;
                }
            } else {
                last_end = m.end;
            }
            last_match = Some(m.end);
            match matched(caps) {
                Ok(true) => return Ok(Ok(())),
                Ok(false) => continue,
                Err(e) => return Ok(Err(e)),
            }
        }
    }
}
