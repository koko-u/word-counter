/// An iterator of words and their number of occurrences.
pub struct FrequencyIter<'a> {
    pub iter: std::collections::hash_map::Iter<'a, String, u32>,
}

impl<'a> Iterator for FrequencyIter<'a> {
    type Item = (&'a str, u32);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(k, v)| (k.as_str(), *v))
    }
}
