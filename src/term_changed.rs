use std::{
  collections::HashMap,
  path::{Path, PathBuf},
};

use tran_term::yml::Term;
use change::Diff;
use aok::Result;

use crate::{TERM, yml_lang};

pub type LangTermPreNow = HashMap<u32, (Term, Term)>;

fn load(path: &Path) -> Result<Term> {
  if path.exists() {
    return tran_term::yml::load(ifs::rstr(path)?);
  }
  Ok(Term::default())
}

// 返回值: 术语有修改的原语言, 差异对象(用来更新差异数据库)
pub fn term_changed(root: &Path) -> Result<(LangTermPreNow, Diff)> {
  let mut set = HashMap::new();
  let dir_term = root.join("term");
  let scan = change::Scan::new(&dir_term, |build| build)?;
  let dir_hash = root.join("hash");
  let diff = scan.diff(dir_hash.join("term.yml"))?;

  let dir_hash_term = dir_hash.join(TERM);

  for (path, _) in &diff.changed {
    let path: PathBuf = path.into();

    let pre = dir_hash_term.join(&path);

    let pre = load(&pre)?;

    let now = load(&dir_term.join(&path))?;

    if now == pre {
      continue;
    }

    if let Some(lang) = yml_lang(&path) {
      set.insert(lang, (pre, now));
    }
  }

  Ok((set, diff))
}
