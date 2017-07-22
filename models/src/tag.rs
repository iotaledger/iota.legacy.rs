use trytes::*;
use alloc::Vec;
use core::fmt;

pub const TAG_LEN_TRITS: usize = 81;

#[derive(Clone, Eq, PartialEq)]
pub struct Tag(Vec<Trit>);

#[derive(Clone, Eq, PartialEq)]
pub struct TagView<'a>(&'a [Trit]);

#[derive(Debug, Eq, PartialEq)]
pub enum TagParseError {
    InvalidLength,
}

impl IntoTrits<Trit> for Tag {
    fn len_trits(&self) -> usize {
        self.0.len()
    }
    fn trits(&self) -> Vec<Trit> {
        self.0.clone()
    }
}

impl FromTrits<Trit> for Tag {
    type Err = TagParseError;
    fn from_trits(base: &[Trit]) -> Result<Self, Self::Err> {
        TagView::from_trits(base).map(|v| v.to_tag())
    }
}

impl Tag {
    pub fn view(&self) -> TagView {
        TagView(self.0.as_slice())
    }
}

impl Default for Tag {
    fn default() -> Self {
        use core::iter;
        Tag(iter::repeat(0).take(TAG_LEN_TRITS).collect())
    }
}
impl<'a> fmt::Display for TagView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = trits_to_string(self.0).unwrap();
        f.write_str(&s)
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.view().fmt(f)
    }
}

impl fmt::Debug for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Tag(")
            .and_then(|_| fmt::Display::fmt(self, f))
            .and_then(|_| f.write_str(")"))
    }
}

impl<'a> fmt::Debug for TagView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TagView(")
            .and_then(|_| fmt::Display::fmt(self, f))
            .and_then(|_| f.write_str(")"))
    }
}


impl<'a> TagView<'a> {
    pub fn from_trits(base: &'a [Trit]) -> Result<Self, TagParseError> {
        if base.len() != TAG_LEN_TRITS {
            return Err(TagParseError::InvalidLength);
        }
        Ok(TagView(base))
    }

    pub fn to_tag(&self) -> Tag {
        Tag(self.0.to_vec())
    }
}
