#![feature(str_as_str)]
#![feature(str_split_remainder)]

mod scan;
pub use scan::scan;

mod error;
pub use error::Error;

// tran_yml 是 路径 -> [源语言->翻译语言]
mod tran_yml;
pub use tran_yml::tran_yml;

mod term_changed;
pub use term_changed::{LangTermPreNow, term_changed};
mod yml_lang;
pub use yml_lang::yml_lang;

mod glob_from_to;
use glob_from_to::GlobFromTo;

// 记录译文对应的原文哈希
mod traned_src;
pub use traned_src::TranedSrc;

mod lang_changed;
pub use lang_changed::lang_changed;

pub mod scan_result;

mod tran;
pub use tran::tran;

pub const DOT_I18N: &str = ".i18n";
pub const TERM: &str = "term";

pub fn dir_hash(root: &std::path::Path) -> std::path::PathBuf {
  root.join(DOT_I18N).join("hash").join("lang")
}
