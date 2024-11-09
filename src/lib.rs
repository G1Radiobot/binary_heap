
///Generic binary heap.
pub struct BinaryHeap<T: Ord>(Vec<T>);

impl<T: Ord> From<Vec<T>> for BinaryHeap<T> {
    fn from(old_vec: Vec<T>) -> Self {
        let mut new_b_heap = BinaryHeap(old_vec);
        for i in (0..new_b_heap.0.len()).rev() {
            new_b_heap.sink(i);
        }
        new_b_heap
    }  
}

impl<T: Ord> BinaryHeap<T> {
    ///Creates an empty binary heap. Preferably, Binary Heaps should be created using "with_capacity", to avoid unnesscary memory allocations.
    pub fn new() -> Self {
        BinaryHeap(Vec::new())
    }

    ///Creates an empty binary heap with the specifed capacity.
    pub fn with_capacity(cap: usize) -> Self {
        let new_vec = Vec::with_capacity(cap);
        BinaryHeap(new_vec)
    }
    
    ///Returns the parent of a node.
    ///TODO Could probably be removed.
    fn parent(index: usize) -> Option<usize> {
        if index == 0 {
            None
        } else {
            Some(((index + 1) / 2) - 1)
        }
    }
    /*
    fn left_child(&self, index: usize) -> Option<usize> {
        let rtn = (2 * (index + 1)) - 1;
        if rtn >= self.0.len() {
            None
        } else {
            Some(rtn)
        }
    }

    fn right_child(&self, index: usize) -> Option<usize> {
        self.left_child(index).map(|mut value| {
            value = value + 1;
            value
        }).filter(|value| {
            *value < self.0.len()
        })
    }

    fn left_child(i: usize) -> usize {
        (2 * (i + 1)) - 1
    }

    fn right_child(i: usize) -> usize {
        Self::left_child(i) + 1
    }
    */

    ///Floats a value up from its current position in the heap.
    fn float(&mut self, i: usize) {
        let me = &mut self.0;
        let mut cur_i = i;

        while let Some(parent) = Self::parent(cur_i) {
            if me.get(parent).zip(me.get(cur_i)).filter(|(parent_t, i_t)| {
                parent_t > i_t
            }).is_some() {
                me.swap(parent, cur_i);
                cur_i = parent;
            } else {
                break
            };
        }
    }

    ///Sinks a value down from its current position in the heap.
    fn sink(&mut self, index: usize) {
        let me = &mut self.0;
        let mut i = index;
        loop {
            let left = i * 2 + 1;
            let right = i * 2 + 1 + 1;
            let new_index;
            match me.get(right).map(|right_value| {
                std::cmp::min(&me[left], right_value)
            }).or(me.get(left)).filter(|&x| {
                *x < me[i]
            }) {
                Some(swap) if *swap == me[left] => new_index = left,
                Some(swap) if *swap == me[right] => new_index = right,
                _ => break,
            }
            me.swap(i, new_index);
            i = new_index;
        }
    }

    /*
    fn sink(&mut self) {
        let me = &mut self.0;
        let mut i = 0;

        while let Some(swap_target) = {
            let lc = Self::left_child(i);
            let rc = Self::right_child(i);
            match (me.get(lc), me.get(rc)) {
                (Some(lv), None) => {
                    if *lv < me[i] {
                        Some(lc)
                    } else {
                        None
                    }
                }
                (Some(lv), Some(rv)) => {
                    if *lv < me[i] || *rv < me[i]{
                        if *rv < *lv {
                            Some(rc)
                        } else {
                            Some(lc)
                        }
                    } else {
                        None
                    }
                }
                (None, None) => {
                    None
                }
                (None, Some(_)) => panic!("Somehow ended up with right child and no left child!"),
            }
        }{
            //println!("sink: Parent: {} swaps with Child: {}", me[i], me[swap_target]);
            me.swap(i, swap_target);
            i = swap_target;
        }
    }
    */

    ///Pushes a value onto the heap, then calls float on that value to maintain binary heap properties.
    pub fn push(&mut self, value: T) {
        self.0.push(value);
        let cur_index = self.0.len() - 1;
        self.float(cur_index);
    }

    ///Pops the root value off the heap, moves the last value of the heap to the root, then calls sink on that value to maintain binary heap properties.
    ///Returns ```None``` if binary heap is empty.
    pub fn pop(&mut self) -> Option<T> {
        if self.0.is_empty() {
            None
        } else {
            let rtn = Some(self.0.swap_remove(0));
            self.sink(0);
            rtn
        }
        
    }
}

#[cfg(test)]
mod test {
    use std::cmp::Reverse;
    use super::BinaryHeap;

    #[test]
    fn push_test() {
        let mut bob = BinaryHeap::<u32>::with_capacity(5);
        bob.push(5);
        bob.push(11);
        bob.push(3);
        bob.push(7);
        bob.push(1);
        bob.push(7);

        assert_eq!(bob.pop(), Some(1));
        assert_eq!(bob.pop(), Some(3));
        bob.push(2);
        assert_eq!(bob.pop(), Some(2));
        assert_eq!(bob.pop(), Some(5));
        assert_eq!(bob.pop(), Some(7));
        assert_eq!(bob.pop(), Some(7));
        assert_eq!(bob.pop(), Some(11));
        assert_eq!(bob.pop(), None);
    }
    #[test]
    fn from_test() {
        let steve = vec![8, 4, 5, 2, 2, 9, 22];
        let mut heap_steve = BinaryHeap::<usize>::from(steve);

        assert_eq!(heap_steve.pop(), Some(2));
        assert_eq!(heap_steve.pop(), Some(2));
        assert_eq!(heap_steve.pop(), Some(4));
        assert_eq!(heap_steve.pop(), Some(5));
        assert_eq!(heap_steve.pop(), Some(8));
        assert_eq!(heap_steve.pop(), Some(9));
        assert_eq!(heap_steve.pop(), Some(22));

        let mut heap_john: BinaryHeap<usize> = vec![2, 3, 1].into();

        assert_eq!(heap_john.pop(), Some(1));
        assert_eq!(heap_john.pop(), Some(2));
        assert_eq!(heap_john.pop(), Some(3));
    }
    #[test]
    fn reverse_test() {
        let joe = vec![Reverse(8), Reverse(2), Reverse(4), Reverse(4), Reverse(5), Reverse(9)];

        let mut heap_joe = BinaryHeap::<Reverse<usize>>::from(joe);

        assert_eq!(heap_joe.pop(), Some(Reverse(9)));
        assert_eq!(heap_joe.pop(), Some(Reverse(8)));
        assert_eq!(heap_joe.pop(), Some(Reverse(5)));
        assert_eq!(heap_joe.pop(), Some(Reverse(4)));
        assert_eq!(heap_joe.pop(), Some(Reverse(4)));
        assert_eq!(heap_joe.pop(), Some(Reverse(2)));

    }
}