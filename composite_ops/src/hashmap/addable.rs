use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Add;

#[derive(Debug, PartialEq, Eq)]
pub struct AddableHashMap<K: Eq + Hash + Clone, V: Add<Output = V> + Clone>(pub HashMap<K, V>);

impl<K: Eq + Hash + Clone, V: Add<Output = V> + Clone> std::ops::Add for AddableHashMap<K, V> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result: HashMap<K, V> = HashMap::new();
        for k in self.0.keys().chain(rhs.0.keys()) {
            if self.0.contains_key(k) && rhs.0.contains_key(k) {
                result.insert(k.clone(), self.0[k].clone() + rhs.0[k].clone());
            } else if self.0.contains_key(k) {
                result.insert(k.clone(), self.0[k].clone());
            } else {
                result.insert(k.clone(), rhs.0[k].clone());
            }
        }
        AddableHashMap(result)
    }
}
