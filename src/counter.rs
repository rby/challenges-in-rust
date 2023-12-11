use std::collections::{hash_map, HashMap};
use std::hash::Hash;
use std::marker::PhantomData;

/// Holds the count of unique elements.
pub struct Counter<'a, T> {
    map: HashMap<T, usize>,
    phantom: PhantomData<&'a T>,
}

impl<'a, T> Counter<'a, T> {
    /// Builds a Counter from an iterator of `T`
    pub fn from<Iter>(iter: Iter) -> Self
    where
        T: Eq + Hash,
        Iter: Iterator<Item = T>,
    {
        let mut c = Counter::new();
        c.insert_all(iter);
        c
    }

    pub fn new() -> Self {
        Counter::<'a, T> {
            map: HashMap::new(),
            phantom: PhantomData,
        }
    }

    pub fn insert(&mut self, elt: T)
    where
        T: Eq + Hash,
    {
        self.map.entry(elt).and_modify(|c| *c += 1).or_insert(1);
    }

    pub fn insert_all<Iter>(&mut self, iter: Iter)
    where
        T: Eq + Hash,
        Iter: Iterator<Item = T>,
    {
        for x in iter {
            self.insert(x);
        }
    }

    pub fn get(&self, k: &T) -> &usize
    where
        T: Eq + Hash,
    {
        self.map.get(k).unwrap_or(&0)
    }
}

impl<'a, T: PartialEq + Eq + Hash> PartialEq for Counter<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        fn included<T, V>(m1: &HashMap<T, V>, m2: &HashMap<T, V>) -> bool
        where
            V: PartialEq + Eq,
            T: PartialEq + Eq + Hash,
        {
            for (k, v) in m1.iter() {
                if !m2.get(k).map_or(false, |v2| v2 == v) {
                    return false;
                }
            }
            return true;
        }
        return included(&self.map, &other.map) && included(&other.map, &self.map);
    }
}

impl<'a, T: 'a> IntoIterator for Counter<'a, T> {
    type Item = (T, usize);
    type IntoIter = hash_map::IntoIter<T, usize>;
    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}

#[cfg(test)]
mod tests {

    use std::collections::HashSet;

    use quickcheck_macros;

    use crate::counter::Counter;
    #[test]
    fn test_initial_1() {
        let c = Counter::from(1..10);
        for (_, v) in c {
            assert_eq!(v, 1)
        }
    }
    #[quickcheck_macros::quickcheck]
    fn prop_keep_counts(cs: Vec<i32>) -> bool {
        let set: HashSet<i32> = HashSet::from_iter(cs.clone());
        let iter = cs.clone();
        let c = Counter::from(iter.iter());
        set.iter()
            .all(|k| cs.iter().filter(|x| *x == k).count() == *c.get(&k))
    }
}
