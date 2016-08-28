# split-iter [![Build Status](https://travis-ci.org/Mixthos/split-iter.svg?branch=master)](https://travis-ci.org/Mixthos/split-iter)

Provides the trait `Splittable`, which allows you to split an iterator
according to a `predicate`.

## [Documentation](http://mixthos.github.com/split-iter)

## Usage

Add to your Cargo.toml:

```toml
[dependencies]
split-iter = "0.1"
```

## Example

```rust
extern crate split_iter;
use split_iter::Splittable;

fn main() {
	let (odd, even) = (1..10).split(|v| v % 2 == 0);

	assert_eq!(odd.collect::<Vec<_>>(), [1,3,5,7,9]);
	assert_eq!(even.collect::<Vec<_>>(), [2,4,6,8]);
}
```
