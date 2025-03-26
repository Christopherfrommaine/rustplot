//! # Helper Module
//!
//! This module provides various helper functions for plotting and other files.
//! Generally, you shouldn't need to reference it in most cases, and it is mostly
//! for internal use only.
//! 
//! One exception:
//! helper::charset is necesary if you want to customize which charachters a plot uses.
//! 
//! ## Philosophy
//! However, I personally have often found a lot of the functions implemented here
//! to be generally useful for other projects (especially in helper::math),
//! so I like keeping the API open.
//! These helpers are seperated from the public plotting code, so I think it's fine
//! to leave it public. Feel free to take a look in case you find some of these
//! functions helpful for your projects!

pub mod math;
pub mod arrays;
pub mod charset;
pub mod axes;
pub mod func_plot_domain;
pub mod rendering;
pub mod mat_plot_lib;
pub mod file;