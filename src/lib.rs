//! # etree
//!
//! `etree` is a DOM library for XML files.

mod etree;
mod etreenode;
mod xpath;

pub use self::etree::{ETree, WriteError, XPathIterator};
pub use self::etreenode::ETreeNode;
