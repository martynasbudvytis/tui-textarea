#![forbid(unsafe_code)]
#![allow(clippy::needless_range_loop)]
#![warn(clippy::dbg_macro, clippy::print_stdout)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

mod cursor;
mod highlight;
mod history;
mod input;
mod scroll;
#[cfg(feature = "search")]
mod search;
mod textarea;
mod util;
mod widget;
mod word;

#[cfg(any(
    feature = "ratatui-crossterm",
    feature = "ratatui-termion",
    feature = "ratatui-your-backend",
))]
use ratatui as tui;
#[cfg(not(any(
    feature = "ratatui-crossterm",
    feature = "ratatui-termion",
    feature = "ratatui-your-backend",
)))]
#[allow(clippy::single_component_path_imports)]
use ratatui;

#[cfg(feature = "crossterm")]
#[allow(clippy::single_component_path_imports)]
use crossterm;
#[cfg(feature = "ratatui-crossterm")]
use crossterm_026 as crossterm;

pub use cursor::CursorMove;
pub use input::{Input, Key};
pub use scroll::Scrolling;
pub use textarea::TextArea;
