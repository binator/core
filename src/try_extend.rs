#[cfg(feature = "alloc")]
use alloc::{
  collections::{
    BTreeMap,
    BTreeSet,
    BinaryHeap,
    LinkedList,
    TryReserveError,
    VecDeque,
  },
  string::String,
  vec::Vec,
};
#[cfg(feature = "hashmap")]
use core::hash::{
  BuildHasher,
  Hash,
};
#[cfg(feature = "alloc")]
use core::iter::{
  self,
  IntoIterator,
};
#[cfg(feature = "hashmap")]
use std::collections::{
  HashMap,
  HashSet,
};

#[cfg(feature = "smallvec")]
use smallvec::SmallVec;

/// Abstracts something which can try extend `&mut self`.
// FIXME https://github.com/rust-lang/rust/issues/48043
// FIXME Add Error
#[cfg(feature = "alloc")]
pub trait TryExtend {
  /// Item stocked in the collection
  type Item;

  /// Used to expect a collection with an IntoIterator object.
  fn try_extend<Items>(&mut self, items: Items) -> Result<(), TryReserveError>
  where
    Items: IntoIterator<Item = Self::Item>;
}

#[cfg(feature = "alloc")]
impl<Item> TryExtend for Vec<Item> {
  type Item = Item;

  fn try_extend<Items>(&mut self, items: Items) -> Result<(), TryReserveError>
  where
    Items: IntoIterator<Item = Item>,
  {
    iter::Extend::extend(self, items);
    Ok(())
  }
}

#[cfg(feature = "alloc")]
impl<Item> TryExtend for VecDeque<Item> {
  type Item = Item;

  fn try_extend<Items>(&mut self, items: Items) -> Result<(), TryReserveError>
  where
    Items: IntoIterator<Item = Self::Item>,
  {
    iter::Extend::extend(self, items);
    Ok(())
  }
}

#[cfg(feature = "alloc")]
impl<Key: Ord, Value> TryExtend for BTreeMap<Key, Value> {
  type Item = (Key, Value);

  fn try_extend<Items>(&mut self, items: Items) -> Result<(), TryReserveError>
  where
    Items: IntoIterator<Item = Self::Item>,
  {
    iter::Extend::extend(self, items);
    Ok(())
  }
}

#[cfg(feature = "alloc")]
impl<Item: Ord> TryExtend for BinaryHeap<Item> {
  type Item = Item;

  fn try_extend<Items>(&mut self, items: Items) -> Result<(), TryReserveError>
  where
    Items: IntoIterator<Item = Self::Item>,
  {
    iter::Extend::extend(self, items);
    Ok(())
  }
}

#[cfg(feature = "hashmap")]
impl<Key: Eq + Hash, Value, Seed: BuildHasher> TryExtend for HashMap<Key, Value, Seed> {
  type Item = (Key, Value);

  fn try_extend<Items>(&mut self, items: Items) -> Result<(), TryReserveError>
  where
    Items: IntoIterator<Item = Self::Item>,
  {
    iter::Extend::extend(self, items);
    Ok(())
  }
}

#[cfg(feature = "hashmap")]
impl<Item: Eq + Hash, Seed: BuildHasher> TryExtend for HashSet<Item, Seed> {
  type Item = Item;

  fn try_extend<Items>(&mut self, items: Items) -> Result<(), TryReserveError>
  where
    Items: IntoIterator<Item = Self::Item>,
  {
    iter::Extend::extend(self, items);
    Ok(())
  }
}

#[cfg(feature = "alloc")]
impl<Item: Ord> TryExtend for BTreeSet<Item> {
  type Item = Item;

  fn try_extend<Items>(&mut self, items: Items) -> Result<(), TryReserveError>
  where
    Items: IntoIterator<Item = Self::Item>,
  {
    iter::Extend::extend(self, items);
    Ok(())
  }
}

#[cfg(feature = "alloc")]
impl<Item> TryExtend for LinkedList<Item> {
  type Item = Item;

  fn try_extend<Items>(&mut self, items: Items) -> Result<(), TryReserveError>
  where
    Items: IntoIterator<Item = Self::Item>,
  {
    iter::Extend::extend(self, items);
    Ok(())
  }
}

#[cfg(feature = "alloc")]
impl TryExtend for String {
  type Item = char;

  fn try_extend<Items>(&mut self, items: Items) -> Result<(), TryReserveError>
  where
    Items: IntoIterator<Item = Self::Item>,
  {
    iter::Extend::extend(self, items);
    Ok(())
  }
}

#[cfg(feature = "smallvec")]
impl<Item, const N: usize> TryExtend for SmallVec<[Item; N]> {
  type Item = Item;

  fn try_extend<Items>(&mut self, items: Items) -> Result<(), TryReserveError>
  where
    Items: IntoIterator<Item = Self::Item>,
  {
    iter::Extend::extend(self, items);
    Ok(())
  }
}

#[cfg(feature = "alloc")]
impl TryExtend for () {
  type Item = ();

  fn try_extend<Items>(&mut self, _items: Items) -> Result<(), TryReserveError>
  where
    Items: IntoIterator<Item = Self::Item>,
  {
    Ok(())
  }
}
