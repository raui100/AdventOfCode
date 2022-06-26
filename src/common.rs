use std::collections::HashMap;
use std::hash::Hash;

pub trait Solution {
    fn name(&self) -> &'static str;
    fn part_a(&self) -> Option<String> { None }
    fn part_b(&self) -> Option<String> { None }
    fn solution(&self) -> String{
        let part_a = self.part_a();
        let part_b = self.part_b();

        let mut out: String = String::new();
        if part_a.is_some() || part_b.is_some() {
            out.push_str(&*format!("Solution for: {}\n", self.name()));
        }

        if let Some(output) = part_a {
            out.push_str(&*format!("Part A: {}\n", output));
        }
        if let Some(output) = part_b {
            out.push_str(&*format!("Part B: {}\n", output));
        }

        return out
    }
}

/// Casts iterator over integers (eg: Vec<u32>
pub fn bin_to_int(vec: Vec<u8>) -> u32 {
    // Asserts that the entries of `vec` are all either 0 or 1
    debug_assert!(vec.clone().iter().all(|n| *n == 0 || *n == 1));
    // Computes the binary number to an u32 by using bit shifting
    vec.into_iter().fold(0_u32, |sum, n| (sum << 1) + n as u32 )
}


/// Counts the number of occurrences of each entry in the iterator
///
/// # Examples
/// ```
/// let input = "aab";
/// let result = HashMap::from([('a', 2), ('b', 1)]);
/// assert_eq!(result, count_frequency(&input.chars()))
/// ```
pub fn count_frequency<I, K>(iter: I) -> HashMap<K, usize>
    where
        I: IntoIterator<Item=K>,
        K: Eq + Hash,
{
    let mut frequency: HashMap<K, usize> = HashMap::new();
    for item in iter {
        *frequency.entry(item).or_insert(0) += 1;
    }

    frequency
}