//
// cyclic_adjacent_pair.rs
// Copyright (C) 2019 Malcolm Ramsay <malramsay64@gmail.com>
// Distributed under terms of the MIT license.
//

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct CyclicAdjacentPairs<I, A>
where
    I: Iterator<Item = A>,
    A: Clone,
{
    iterator: I,
    prev_item: Option<A>,
    first_item: Option<A>,
}

impl<I, A> CyclicAdjacentPairs<I, A>
where
    I: Iterator<Item = A>,
    A: Clone,
{
    fn new(iterator: I) -> CyclicAdjacentPairs<I, A> {
        CyclicAdjacentPairs {
            iterator,
            prev_item: None,
            first_item: None,
        }
    }
}

impl<I, A> Iterator for CyclicAdjacentPairs<I, A>
where
    I: Iterator<Item = A>,
    A: Clone,
{
    type Item = (A, A);

    fn next(&mut self) -> Option<Self::Item> {
        let (prev_item, current_item) = match (self.prev_item.take(), self.iterator.next()) {
            // The common case
            (Some(prev), Some(current)) => (prev, current),
            // Iterator has run out, so use first value
            (Some(prev), None) => (prev, self.first_item.take()?),
            // Prev has not been set, also initial value not yet set
            (None, Some(current)) => {
                let next = self.iterator.next()?;
                self.first_item = Some(current.clone());
                (current, next)
            }
            _ => return None,
        };

        self.prev_item = Some(current_item.clone());

        Some((prev_item, current_item))
    }
}

pub trait CyclicAdjacentPairIterator {
    type Item: Clone;
    type Iterator: Iterator<Item = Self::Item>;

    fn cyclic_adjacent_pairs(self) -> CyclicAdjacentPairs<Self::Iterator, Self::Item>;
}

impl<I, A> CyclicAdjacentPairIterator for I
where
    I: Iterator<Item = A>,
    A: Clone,
{
    type Item = A;
    type Iterator = Self;

    fn cyclic_adjacent_pairs(self) -> CyclicAdjacentPairs<Self::Iterator, Self::Item> {
        CyclicAdjacentPairs::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::CyclicAdjacentPairIterator;

    #[test]
    fn should_provide_nothing_without_items() {
        let array: [i32; 0] = [];
        let mut iterator = array.iter().cyclic_adjacent_pairs();

        assert_eq!(None, iterator.next());
    }

    #[test]
    fn should_provide_nothing_for_only_one_input() {
        let array = [1];
        let mut iterator = array.iter().cyclic_adjacent_pairs();

        assert_eq!(None, iterator.next());
    }

    #[test]
    fn should_provide_two_pairs_for_two_inputs() {
        let array = [1, 2];
        let mut iterator = array.iter().cyclic_adjacent_pairs();

        assert_eq!(Some((&1, &2)), iterator.next());
        assert_eq!(Some((&2, &1)), iterator.next());
        assert_eq!(None, iterator.next());
    }

    #[test]
    fn should_provide_three_pairs_for_three_inputs() {
        let array = [1, 2, 3];
        let mut iterator = array.iter().cyclic_adjacent_pairs();

        assert_eq!(Some((&1, &2)), iterator.next());
        assert_eq!(Some((&2, &3)), iterator.next());
        assert_eq!(Some((&3, &1)), iterator.next());
        assert_eq!(None, iterator.next());
    }

    #[test]
    fn should_provide_many_pairs() {
        let array = [1, 2, 3, 4, 5, 6];
        let mut iterator = array.iter().cyclic_adjacent_pairs();

        assert_eq!(Some((&1, &2)), iterator.next());
        assert_eq!(Some((&2, &3)), iterator.next());
        assert_eq!(Some((&3, &4)), iterator.next());
        assert_eq!(Some((&4, &5)), iterator.next());
        assert_eq!(Some((&5, &6)), iterator.next());
        assert_eq!(Some((&6, &1)), iterator.next());
        assert_eq!(None, iterator.next());
    }

    #[test]
    fn should_work_with_into_iterator() {
        let vector = vec![1, 2, 3];
        let mut iterator = vector.into_iter().cyclic_adjacent_pairs();

        assert_eq!(Some((1, 2)), iterator.next());
        assert_eq!(Some((2, 3)), iterator.next());
        assert_eq!(Some((3, 1)), iterator.next());
        assert_eq!(None, iterator.next());
    }
}
