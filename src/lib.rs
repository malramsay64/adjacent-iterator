//
// lib.rs
// Copyright (C) 2019 Malcolm Ramsay <malramsay64@gmail.com>
// Distributed under terms of the MIT license.
//

pub mod adjacent_pair;
pub mod cyclic_adjacent_pair;

pub use crate::adjacent_pair::AdjacentPairIterator;
pub use crate::cyclic_adjacent_pair::CyclicAdjacentPairIterator;
