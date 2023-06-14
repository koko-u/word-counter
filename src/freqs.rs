//! frequency of occurrences

use core::fmt;
use std::collections::HashMap;

use self::builder::FrequencyBuilder;

/// Keeps a hash table of frequency of occurrence
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Frequency(HashMap<String, u32>);

impl Frequency {
    /// Generate the builder that calculates the frequency of occurrence for each unit
    pub fn builder(s: String) -> FrequencyBuilder {
        FrequencyBuilder(s)
    }

    /// Retrieves the hash table held internally
    pub fn into_inner(self) -> HashMap<String, u32> {
        self.0
    }

    /// Merge the frequency of occurrence from an iterator that has the word and its number of occurrences as items
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

    /// Get an iterator that retrieves words and their frequencies in order
    pub fn iter(&self) -> impl Iterator<Item = (&str, u32)> {
        iter::FrequencyIter {
            iter: self.0.iter(),
        }
    }
}

impl fmt::Display for Frequency {
    /// Outputs word frequencies in descending order in histgram format
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
