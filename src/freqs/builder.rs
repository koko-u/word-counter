//
// let s = "ab cd ef";
// let freq = Frequency::builder(s).by(CountUnit::Word)
//            ~~~~~~~~~~~~~~~~~~~~~
//              FrequencyBuilder
//

use std::collections::HashMap;

use crate::opts::CountUnit;

use super::Frequency;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrequencyBuilder(pub String);

impl FrequencyBuilder {
    pub fn by(self, _unit: CountUnit) -> Frequency {
        let mut freq = HashMap::<String, u32>::new();
        for word in self.0.split_whitespace() {
            freq.entry(word.into()).and_modify(|c| *c += 1).or_insert(1);
        }

        Frequency(freq)
    }
}
