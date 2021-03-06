use storage::*;
use std::collections::HashMap;

pub struct HashStorage {
    elements: Box<HashMap<u16, Vec<StorageItem>>>,
}

impl Storage for HashStorage {
    fn new() -> Self {
        HashStorage {
            elements: Box::new(HashMap::new()),
        }
    }

    fn push(&mut self, priority: u16, payload: Box<Vec<u8>>) {
        let storage_item = StorageItem {
            priority: priority,
            data: payload,
        };
        match self.elements.contains_key(&priority) {
            true => self.elements.get_mut(&priority).unwrap().push(storage_item),
            false => {
                self.elements.insert(priority, vec![storage_item]);
            }
        };
    }

    fn pop(&mut self, count: usize) -> Option<Vec<StorageItem>> {
        let mut found = 0;
        let mut items: Vec<StorageItem> = Vec::with_capacity(count);

        while found < count {
            match self.max_priority() {
                Some(priority) => {
                    let mut priority_elements = self.elements.get_mut(&priority).unwrap();
                    let missing_elements = count - found;
                    let len = priority_elements.len();
                    let range_end = match missing_elements > len {
                        true => len,
                        false => missing_elements,
                    };

                    let mut drained_elements: Vec<StorageItem> =
                        priority_elements.drain(0..range_end).collect();
                    items.append(&mut drained_elements);
                    found += range_end;
                }
                None => break,
            }
        }
        match found == 0 {
            true => None,
            false => Some(items),
        }
    }

    fn max_priority(&self) -> Option<u16> {
        let mut max_priority: u16 = 0;
        let mut found = false;

        for (priority, elements) in self.elements.iter() {
            if !elements.is_empty() {
                found = true;
                if *priority > max_priority {
                    max_priority = *priority
                }
            }
        }

        match found {
            true => Some(max_priority),
            false => None,
        }
    }

    fn dump(&self) {}
    fn load(&mut self) {}

    fn clear(&mut self) {
        self.elements.clear();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works_for_no_elements() {
        let mut instance = HashStorage::new();
        let elems = instance.pop(1);
        assert_eq!(elems.is_none(), true);

        let elems = instance.pop(0);
        assert_eq!(elems.is_none(), true);
    }

    #[test]
    fn it_works_for_a_single_element() {
        let mut instance = HashStorage::new();
        instance.push(10, Box::new(vec![2]));
        let elems = instance.pop(1).unwrap();
        assert_eq!(elems.len(), 1);
        let storage_item = elems.get(0).unwrap();
        assert_eq!(storage_item.data, Box::new(vec![2]));
        assert_eq!(storage_item.priority, 10);
    }

    #[test]
    fn it_works_for_2_elements() {
        let mut instance = HashStorage::new();
        instance.push(2, Box::new(vec![1]));
        instance.push(10, Box::new(vec![2]));
        let elems = instance.pop(2).unwrap();
        assert_eq!(elems.len(), 2);

        let storage_item1 = elems.get(0).unwrap();
        assert_eq!(storage_item1.data, Box::new(vec![2]));
        assert_eq!(storage_item1.priority, 10);

        let storage_item2 = elems.get(1).unwrap();
        assert_eq!(storage_item2.data, Box::new(vec![1]));
        assert_eq!(storage_item2.priority, 2);
    }

    #[test]
    fn it_works_for_more_elements() {
        let mut instance = HashStorage::new();
        instance.push(2, Box::new(vec![1]));

        let elems = instance.pop(100).unwrap();
        assert_eq!(elems.len(), 1);

        let storage_item1 = elems.get(0).unwrap();
        assert_eq!(storage_item1.data, Box::new(vec![1]));
        assert_eq!(storage_item1.priority, 2);
    }

    #[test]
    fn it_preserves_order_for_the_same_priority() {
        let mut instance = HashStorage::new();
        instance.push(2, Box::new(vec![1]));
        instance.push(2, Box::new(vec![2]));
        instance.push(2, Box::new(vec![3]));
        instance.push(4, Box::new(vec![4]));

        let elems = instance.pop(4).unwrap();

        let storage_item1 = elems.get(0).unwrap();
        assert_eq!(storage_item1.data, Box::new(vec![4]));
        assert_eq!(storage_item1.priority, 4);

        let storage_item2 = elems.get(1).unwrap();
        assert_eq!(storage_item2.data, Box::new(vec![1]));
        assert_eq!(storage_item2.priority, 2);

        let storage_item3 = elems.get(2).unwrap();
        assert_eq!(storage_item3.data, Box::new(vec![2]));
        assert_eq!(storage_item3.priority, 2);

        let storage_item4 = elems.get(3).unwrap();
        assert_eq!(storage_item4.data, Box::new(vec![3]));
        assert_eq!(storage_item4.priority, 2);
    }

    #[test]
    fn it_returns_correct_max_priority() {
        let mut instance = HashStorage::new();

        instance.push(10, Box::new(vec![1]));
        instance.push(2, Box::new(vec![1]));
        instance.push(11, Box::new(vec![1]));
        instance.push(3, Box::new(vec![1]));
        instance.push(0, Box::new(vec![1]));
        assert_eq!(instance.max_priority(), Some(11));
    }

    #[test]
    fn it_clears() {
        let mut instance = HashStorage::new();
        instance.push(10, Box::new(vec![1]));
        instance.clear();
        let opt = instance.pop(1);
        assert_eq!(opt.is_none(), true);
    }
}
