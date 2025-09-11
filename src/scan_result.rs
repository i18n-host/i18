use std::{collections::HashMap, path::PathBuf};

use lang::CODE;
use aok::{OK, Void};
use change::Diff;
use tran_api::{Tran, Update};

use crate::{LangTermPreNow, TERM, TranedSrc, dir_hash};

pub struct ScanResult {
  pub update_li: Vec<Update>,
  pub tran_li: Vec<Tran>, // from_lang txt to_lang
  pub save: Save,
}

pub struct Save {
  pub root: PathBuf,
  pub traned_src: TranedSrc,
  pub lang_diff: HashMap<u32, Diff>,
  pub term_diff: Diff,
  pub term_pre_now: LangTermPreNow,
  pub dir_i18n: PathBuf,
}

impl Save {
  pub fn refresh(&mut self, lang: usize, path: &str) -> Void {
    let diff = self.lang_diff.entry(lang as u32).or_insert_with(|| {
      let code = CODE[lang];
      Diff::new(
        self.root.join(code),
        dir_hash(&self.root).join(format!("{code}.yml")),
      )
    });
    diff.refresh(path)?;
    OK
  }

  pub fn save_lang(&self, lang: u32) {
    if let Some(diff) = self.lang_diff.get(&lang) {
      xerr::log!(diff.save());
    }
    if self.term_pre_now.contains_key(&lang) {
      let filename = format!("{}.yml", CODE[lang as usize]);
      // 备份术语表, 当术语表改变的时候, 可以判断文章是否需要重新翻译
      xerr::log!(ifs::cp(
        self.dir_i18n.join(TERM).join(&filename),
        self.dir_i18n.join("hash").join(TERM).join(filename),
      ));
    }
  }

  pub fn save(self) -> Void {
    self.term_diff.save()?;
    self.traned_src.save()?;
    for i in self.lang_diff.values() {
      i.save()?;
    }
    OK
  }
}
