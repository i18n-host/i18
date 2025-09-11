use std::{collections::HashMap, path::Path};

use change::Diff;
use aok::Result;

use crate::dir_hash;

pub fn lang_changed(
  root: &Path,
  lang_set: impl IntoIterator<Item = u32>,
) -> Result<HashMap<u32, Diff>> {
  let dir_hash = dir_hash(root);
  if !dir_hash.exists() {
    std::fs::create_dir_all(&dir_hash)?;
  }
  let mut lang_change = HashMap::new();

  for lang in lang_set {
    let lang_en = lang::CODE[lang as usize];
    let scan = change::Scan::new(root.join(lang_en), |build| {
      use index_of::IndexOf;
      build.filter_entry(|entry| {
        if entry.path().is_dir() {
          return true;
        }
        if let Some(ext) = entry.path().extension() {
          let ext = ext.to_string_lossy();
          return tran_api::EXT.index_of(&ext.as_str()).is_some();
        }
        false
      })
    })?;
    let diff = scan.diff(dir_hash.join(format!("{lang_en}.yml")))?;
    lang_change.insert(lang, diff);
  }
  Ok(lang_change)
}
