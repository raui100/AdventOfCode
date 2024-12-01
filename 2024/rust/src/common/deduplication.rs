use std::{collections::BTreeMap, ops::Deref};

pub struct Deduplicated<K>(BTreeMap<K, usize>);

impl<K: Ord> Deduplicated<K> {
    /// Iterates over the sorted original values
    pub fn duplicated(&self) -> impl Iterator<Item = &'_ K> {
        self.0.iter().flat_map(|(k, &v)| std::iter::repeat_n(k, v))
    }
}

impl<K> Deref for Deduplicated<K> {
    type Target = BTreeMap<K, usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K: Ord, I> From<I> for Deduplicated<K>
where
    I: IntoIterator<Item = K>,
{
    fn from(value: I) -> Self {
        let mut occurences = BTreeMap::new();
        for key in value {
            *occurences.entry(key).or_default() += 1;
        }
        Self(occurences)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn deduplication() {
        let nums = vec![1u8, 2, 1];
        let dedup = Deduplicated::from(nums.clone());
        assert_eq!(dedup.get(&1), Some(&2));
        assert_eq!(dedup.get(&2), Some(&1));
        assert_eq!(dedup.len(), 2);
        assert_eq!(
            dedup.duplicated().copied().collect::<Vec<u8>>(),
            vec![1, 1, 2]
        );
    }
}
