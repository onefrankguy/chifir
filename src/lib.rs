//! Chifir is a virtual computer based on the Tien Nguyen and Alan Kay's paper,
//! ["The Cuneiform Tablets of 2015"][paper].
//!
//!
//! [paper]: http://www.vpri.org/pdf/tr2015004_cuneiform.pdf "Tien Nguyen and Alan Kay (Viewpoints Research Insitute): The Cuneiform Tablets of 2015"
extern crate termion;

pub mod computer;
mod sixel;
pub mod compiler;
