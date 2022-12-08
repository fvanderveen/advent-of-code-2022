pub trait CollectionExtension {
    fn deduplicate(&self) -> Self;
    fn union(&self, other: &Self) -> Self;
    fn push_all(&mut self, other: &Self);
}

impl<T> CollectionExtension for Vec<T> where T: Clone + Eq {
    fn deduplicate(&self) -> Self {
        let mut result = vec![];
        for item in self {
            if !result.contains(item) { result.push(item.clone()) }
        }
        result
    }

    fn union(&self, other: &Self) -> Self {
        self.iter().cloned().filter(|v| other.contains(v)).collect()
    }

    fn push_all(&mut self, other: &Self) {
        for value in other {
            self.push(value.clone());
        }
    }
}