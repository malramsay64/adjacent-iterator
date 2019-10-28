//
// adjacent_pair.rs
// Copyright (C) 2019 Malcolm Ramsay <malramsay64@gmail.com>
// Distributed under terms of the MIT license.
//

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct AdjacentPairs<I, A>
where
    I: Iterator<Item = A>,
    A: Clone,
{
    iterator: I,
    last_item: Option<A>,
}

impl<I, A> AdjacentPairs<I, A>
where
    I: Iterator<Item = A>,
    A: Clone,
{
    fn new(iterator: I) -> AdjacentPairs<I, A> {
        AdjacentPairs {
            iterator,
            last_item: None,
        }
    }
}

impl<I, A> Iterator for AdjacentPairs<I, A>
where
    I: Iterator<Item = A>,
    A: Clone,
{
    type Item = (A, A);

    fn next(&mut self) -> Option<Self::Item> {
        let last_item = match self.last_item.take() {
            Some(item) => item,
            None => self.iterator.next()?,
        };

        let current_item = self.iterator.next()?;
        self.last_item = Some(current_item.clone());
        Some((last_item, current_item))
    }
}

pub trait AdjacentPairIterator {
    type Item: Clone;
    type Iterator: Iterator<Item = Self::Item>;

    fn adjacent_pairs(self) -> AdjacentPairs<Self::Iterator, Self::Item>;
}

impl<I, A> AdjacentPairIterator for I
where
    I: Iterator<Item = A>,
    A: Clone,
{
    type Item = A;
    type Iterator = Self;

    fn adjacent_pairs(self) -> AdjacentPairs<Self::Iterator, Self::Item> {
        AdjacentPairs::<Self::Iterator, Self::Item>::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::AdjacentPairIterator;

    #[test]
    fn should_provide_nothing_without_items() {
        let array: [i32; 0] = [];
        let mut iterator = array.iter().adjacent_pairs();

        assert_eq!(None, iterator.next());
    }

    #[test]
    fn should_provide_nothing_for_only_one_input() {
        let array = [1];
        let mut iterator = array.iter().adjacent_pairs();

        assert_eq!(None, iterator.next());
    }

    #[test]
    fn should_provide_pair_for_two_inputs() {
        let array = [1, 2];
        let mut iterator = array.iter().adjacent_pairs();

        assert_eq!(Some((&1, &2)), iterator.next());
        assert_eq!(None, iterator.next());
    }

    #[test]
    fn should_provide_two_pairs_for_three_inputs() {
        let array = [1, 2, 3];
        let mut iterator = array.iter().adjacent_pairs();

        assert_eq!(Some((&1, &2)), iterator.next());
        assert_eq!(Some((&2, &3)), iterator.next());
        assert_eq!(None, iterator.next());
    }

    #[test]
    fn should_provide_many_pairs() {
        let array = [1, 2, 3, 4, 5, 6];
        let mut iterator = array.iter().adjacent_pairs();

        assert_eq!(Some((&1, &2)), iterator.next());
        assert_eq!(Some((&2, &3)), iterator.next());
        assert_eq!(Some((&3, &4)), iterator.next());
        assert_eq!(Some((&4, &5)), iterator.next());
        assert_eq!(Some((&5, &6)), iterator.next());
        assert_eq!(None, iterator.next());
    }

    #[test]
    fn should_work_with_into_iterator() {
        let vector = vec![1, 2, 3];
        let mut iterator = vector.into_iter().adjacent_pairs();

        assert_eq!(Some((1, 2)), iterator.next());
        assert_eq!(Some((2, 3)), iterator.next());
        assert_eq!(None, iterator.next());
    }
}
