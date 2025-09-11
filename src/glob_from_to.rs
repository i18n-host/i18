use std::collections::HashSet;

use globset::GlobMatcher;
use ft::FromTo;

pub struct GlobFromTo(pub Vec<(GlobMatcher, Vec<FromTo>)>);

impl GlobFromTo {
  pub fn get(&self, lang: u32, path: impl AsRef<str>) -> Option<Vec<u32>> {
    let path = path.as_ref();
    for (glob, map) in &self.0 {
      if glob.is_match(path) {
        for ft in map {
          if ft.from_lang == lang {
            return Some(ft.to_lang_li.clone());
          }
        }
        return None;
      }
    }

    None
  }

  pub fn lang_set(&self) -> Vec<u32> {
    use lang::LANG;
    let mut lang_set = HashSet::new();

    for i in &self.0 {
      for j in &i.1 {
        if j.to_lang_li.is_empty() {
          return LANG.to_vec();
        }
        lang_set.insert(j.from_lang);
        for k in &j.to_lang_li {
          lang_set.insert(*k);
        }
      }
    }
    lang_set.into_iter().collect::<Vec<_>>()
  }
}
