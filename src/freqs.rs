//! frequency of occurrences

use core::fmt;
use std::collections::HashMap;

use self::builder::FrequencyBuilder;

/// Keeps a hash table of frequency of occurrence
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Frequency(HashMap<String, u32>);

impl Frequency {
    /// Generate the builder that calculates the frequency of occurrence for each unit
    ///
    /// # Example count chars
    ///
    /// ```
    /// use word_counter::freqs::Frequency;
    /// use word_counter::opts::CountUnit;
    ///
    /// let s = "aabcb".to_string();
    /// let freq = Frequency::builder(s).by(CountUnit::Char);
    /// let count = freq.into_inner();
    ///
    /// assert_eq!(count.get("a"), Some(&2));
    /// assert_eq!(count.get("x"), None);
    /// ```
    ///
    /// # Example count words
    ///
    /// ```
    /// use word_counter::freqs::Frequency;
    /// use word_counter::opts::CountUnit;
    ///
    /// let s = "ab xy ab cd".to_string();
    /// let freq = Frequency::builder(s).by(CountUnit::Word);
    /// let count = freq.into_inner();
    ///
    /// assert_eq!(count.get("ab"), Some(&2));
    /// assert_eq!(count.get("01"), None);
    /// ```
    ///
    /// # Example count lines
    ///
    /// ```
    /// use word_counter::freqs::Frequency;
    /// use word_counter::opts::CountUnit;
    ///
    /// let s = r#"ab
    /// cd
    /// ab
    /// xy"#.to_string();
    /// let freq = Frequency::builder(s).by(CountUnit::Line);
    /// let count = freq.into_inner();
    ///
    /// assert_eq!(count.get("ab"), Some(&2));
    /// assert_eq!(count.get("ab\ncd"), None);
    /// ```
    pub fn builder(s: String) -> FrequencyBuilder {
        FrequencyBuilder(s)
    }

    /// Retrieves the hash table held internally
    pub fn into_inner(self) -> HashMap<String, u32> {
        self.0
    }

    /// Merge the frequency of occurrence from an iterator that has the word and its number of occurrences as items
    ///
    /// # examples
    ///
    /// ```
    /// use word_counter::freqs::Frequency;
    ///
    /// let mut freq = Frequency::default();
    /// freq.merge([
    ///     ("ab", 1),
    ///     ("xy", 2),
    /// ]);
    /// freq.merge([
    ///     ("ab", 3),
    ///     ("cd", 4),
    /// ]);
    ///
    /// let count = freq.into_inner();
    /// assert_eq!(count.get("ab"), Some(&4));
    /// assert_eq!(count.get("xy"), Some(&2));
    /// assert_eq!(count.get("cd"), Some(&4));
    /// assert_eq!(count.get("01"), None);
    /// ```
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
    ///
    /// # examples
    ///
    /// ```
    /// use word_counter::freqs::Frequency;
    /// use word_counter::opts::CountUnit;
    ///
    /// let s = "ab cd ab xy".to_string();
    /// let freq = Frequency::builder(s).by(CountUnit::Word);
    ///
    /// let mut freq_vec = freq.iter().collect::<Vec<_>>();
    /// freq_vec.sort_by(|(k1, _), (k2, _)| k1.cmp(&k2));
    ///
    /// assert_eq!(freq_vec, vec![("ab", 2), ("cd", 1), ("xy", 1)]);
    ///
    /// ```
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
                first = false;
            } else {
                writeln!(f)?;
            }
            write!(f, "{word:width$}:{}", "*".repeat(count as usize))?;
        }

        Ok(())
    }
}

mod builder;
mod iter;

#[cfg(test)]
mod tests {
    use std::io;

    use crate::opts::CountUnit;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn display_frequency() -> Result<(), Box<dyn std::error::Error>> {
        use std::io::Write;

        let s = "ab xy ab cd xy ab".to_string();
        let freq = Frequency::builder(s).by(CountUnit::Word);

        let mut writer = io::Cursor::new(Vec::new());
        write!(writer, "{freq}")?;

        let out = String::from_utf8(writer.into_inner())?;
        assert_eq!(
            out,
            r#"ab:***
xy:**
cd:*"#
        );

        Ok(())
    }
}
