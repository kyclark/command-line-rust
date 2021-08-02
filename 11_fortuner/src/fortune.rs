use std::cmp::Ordering;

#[derive(Debug, Eq)]
pub struct Fortune {
    pub source: String,
    pub text: String,
}

impl Ord for Fortune {
    fn cmp(&self, other: &Self) -> Ordering {
        self.source
            .cmp(&other.source)
            .then(self.text.cmp(&other.text))
    }
}

impl PartialOrd for Fortune {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Fortune {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source && self.text == other.text
    }
}
