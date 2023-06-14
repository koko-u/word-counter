use core::fmt;
use std::collections::HashMap;

use self::builder::FrequencyBuilder;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Frequency(HashMap<String, u32>);

impl Frequency {
    pub fn builder(s: String) -> FrequencyBuilder {
        FrequencyBuilder(s)
    }
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

impl fmt::Display for Frequency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ordered = self
            .0
            .iter()
            .map(|(k, v)| (k.as_str(), *v))
            .collect::<Vec<_>>();
        ordered.sort_by(|(_, count1), (_, count2)| count2.cmp(count1));

        let width = ordered
            .iter()
            .map(|(word, _)| word.chars().count())
            .max()
            .unwrap_or(10);

        let mut first = true;
        for (word, count) in ordered {
            if first {
                write!(f, "{word:width$}:{}", "*".repeat(count as usize))?;
                first = false;
            }
            write!(f, "\n{word:width$}:{}", "*".repeat(count as usize))?;
        }

        Ok(())
    }
}

mod builder;
mod iter;
