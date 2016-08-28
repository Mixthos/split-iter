//! Provides the trait `Splittable`, which allows you to split an iterator
//! according to a `predicate`.
//!
//! # Example
//!
//! ```
//! use split_iter::Splittable;
//!
//! fn main() {
//! 	let (odd, even) = (1..10).split(|v| v % 2 == 0);
//!
//! 	assert_eq!(odd.collect::<Vec<_>>(), [1,3,5,7,9]);
//! 	assert_eq!(even.collect::<Vec<_>>(), [2,4,6,8]);
//! }
//! ```


use std::rc::Rc;
use std::collections::VecDeque;
use std::cell::RefCell;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Error as FmtError;


/// Shared inner state for two `Split`s.
struct SharedSplitState<I, P> where
	I: Iterator,
	P: FnMut(&I::Item) -> bool
{
	/// Inner iterator.
	iter: I,
	/// Predicate that chooses whether an item
	/// goes left (`false`) or right (`true`).
	predicate: P,
	/// Cache that saves items that have been skipped by one `Split`.
	/// They will be returned next for the other `Split`.
	cache: VecDeque<I::Item>,
	/// Is the cache currently saving items for the left or for the right split?
	is_right_cached: bool,
}

impl<I, P> SharedSplitState<I, P> where
	I: Iterator,
	P: FnMut(&I::Item) -> bool
{
	/// Creates shared inner state for two `Split`s.
	fn new(iter: I, predicate: P) -> SharedSplitState<I, P> {
		SharedSplitState {
			iter: iter,
			predicate: predicate,
			cache: VecDeque::new(),
			is_right_cached: false,
		}
	}
	
	/// Returns next item for the given `Split`.
	fn next(&mut self, is_right: bool) -> Option<I::Item> {
		// Use cache for correct side
		if is_right == self.is_right_cached {
			if let Some(next) = self.cache.pop_front() {
				return Some(next);
			}
		}
		
		// From inner iterator
		while let Some(next) = self.iter.next() {
			if (self.predicate)(&next) == is_right {
				return Some(next);
			} else {
				// Fill cache with elements for opposite iterator
				self.is_right_cached = !is_right;
				self.cache.push_back(next);
			}
		}
		
		// No element found
		None
	}
}


/// One of a pair of iterators. One returns the items for which the predicate
/// returns `false`, the other one returns the items for which the predicate
/// returns `true`.
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct Split<I, P> where
	I: Iterator,
	P: FnMut(&I::Item) -> bool
{
	/// Shared state with the opposite iterator.
	shared: Rc<RefCell<SharedSplitState<I, P>>>,
	/// Is the iterator the right one or the left one?
	is_right: bool,
}

impl<I, P> Iterator for Split<I, P> where
	I: Iterator,
	P: FnMut(&I::Item) -> bool
{
	type Item = I::Item;
	
	fn next(&mut self) -> Option<I::Item> {
		self.shared.borrow_mut().next(self.is_right)
	}
}

impl<I, P> Debug for Split<I, P> where
	I: Iterator + Debug,
	P: FnMut(&I::Item) -> bool
{
	fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
		fmt.debug_struct("Split")
			.field("iter", &self.shared.borrow().iter)
			.finish()
	}
}


/// Provides an iterator adaptor method that splits an iterator into two
/// iterators according to a predicate.
pub trait Splittable<I> where
	I: Iterator
{
	/// Splits the iterator. The left iterator iterates over all items for which
	/// the `predicate` returns `false`. The right iterator returns all items
	/// for which the `predicate` returns `true`.
	fn split<P>(self, predicate: P) -> (Split<I, P>, Split<I, P>)
		where P: FnMut(&I::Item) -> bool;
}

impl<I> Splittable<I> for I where
	I: Iterator
{
	fn split<P>(self, predicate: P) -> (Split<I, P>, Split<I, P>)
		where P: FnMut(&I::Item) -> bool
	{
		let shared = Rc::new(
			RefCell::new(
				SharedSplitState::new(self, predicate)
			)
		);
		
		let left = Split {
			shared: shared.clone(),
			is_right: false,
		};
		
		let right = Split {
			shared: shared,
			is_right: true,
		};
		
		(left, right)
	}
}


#[cfg(test)]
mod tests {
	use super::Splittable;
	
    #[test]
    fn it_works() {
		let (odd, even) = (1..10).split(|v| v % 2 == 0);
		assert_eq!(odd.collect::<Vec<_>>(), [1,3,5,7,9]);
		assert_eq!(even.collect::<Vec<_>>(), [2,4,6,8]);
		
		let (low, high) = (1..20).split(|v| v >= &10);
		assert_eq!(high.collect::<Vec<_>>(), (10..20).collect::<Vec<_>>());
		assert_eq!(low.collect::<Vec<_>>(), (1..10).collect::<Vec<_>>());
    }
}
