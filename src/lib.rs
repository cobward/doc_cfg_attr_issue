#![feature(doc_auto_cfg)]

#[derive(Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Test();
