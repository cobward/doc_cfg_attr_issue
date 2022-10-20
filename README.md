# Issue with doc_cfg and cfg_attr.

When conditionally deriving a trait using `cfg_attr`, the feature gate is not documented.

(present in 1.66.0-nightly)

## To reproduce:

* Build the documentation of this project with `cargo +nightly doc --all-features --open`.
* View the documentation for the struct `Test`.
* `Debug` should be documentated as only active with the `debug` feature flag set.

## Example:

```rust
#![feature(doc_auto_cfg)]

#[derive(Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Test();
```
![example](https://user-images.githubusercontent.com/93085583/196917554-cb5d2cdf-7344-45e6-bd24-7ded27a0ab8c.png)
