use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Frequency(HashMap<String, u32>);

impl From<String> for Frequency {
    fn from(value: String) -> Self {
        let mut freq = HashMap::<String, u32>::new();
        for word in value.split_whitespace() {
            freq.entry(word.into()).and_modify(|c| *c += 1).or_insert(1);
        }

        Self(freq)
    }
}

impl Frequency {
    pub fn into_inner(self) -> HashMap<String, u32> {
        self.0
    }
    pub fn merge<'a, I>(&mut self, items: I)
    where
        I: IntoIterator<Item = (&'a str, u32)>,
    {
        for (word, count) in items.into_iter() {
            self.0
                .entry(word.into())
                .and_modify(|c| *c += count)
                .or_insert(count);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, u32)> {
        iter::FrequencyIter {
            iter: self.0.iter(),
        }
    }
}

mod iter;
