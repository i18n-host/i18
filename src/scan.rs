use std::path::PathBuf;

use aok::Result;

use crate::{
  DOT_I18N, lang_changed,
  scan_result::{Save, ScanResult},
  term_changed,
};

pub fn scan(root: impl Into<PathBuf>) -> Result<ScanResult> {
  let root = root.into();
  let dir_i18n = root.join(DOT_I18N);

  let tran_glob = crate::tran_yml(&dir_i18n)?;

  // 记录译文的原文
  let mut traned_src = crate::traned_src::load(dir_i18n.join("src.yml"))?;

  // 语言 - 改动的文件
  let lang_diff = lang_changed(&root, tran_glob.lang_set())?;

  let mut update_li: std::collections::HashMap<(u32, u32), Vec<tran_api::UpdateFile>> =
    std::collections::HashMap::new();
  // 找到有修改的术语表(术语表修改后要全部重新翻译)
  let (mut term_pre_now, term_diff) = term_changed(&dir_i18n)?;

  let mut tran_li = Vec::new();
  for (lang, diff) in &lang_diff {
    let lang = *lang;
    let dir = root.join(lang::CODE[lang as usize]);

    let rstr = |path: &String| -> std::io::Result<String> {
      let path = dir.join(path);
      ifs::rstr(path)
    };

    let mut tran_li_push = |path: &String, to_lang_li: Vec<u32>| {
      if let Ok(txt) = xerr::ok!(rstr(path)) {
        tran_li.push(tran_api::Tran {
          from_lang: lang,
          to_lang_li,
          txt,
          path: path.into(),
        });
      }
    };

    // 如果术语表发生改变, 并且存在未改动的源语言文件
    if !diff.no_change.is_empty()
      && let Some((term_pre, term_now)) = term_pre_now.get_mut(&lang)
    {
      for (path, _) in &diff.no_change {
        if let Some(to_li) = tran_glob.get(lang, path) {
          let txt = ifs::rstr(diff.root.join(path)).unwrap_or_default();

          if term_pre.replace(lang, &txt) == term_now.replace(lang, &txt) {
            continue;
          }

          tran_li_push(path, to_li);
        }
      }
    }

    for path in diff.changed.iter().map(|i| &i.0) {
      // 如果译文有对应原文hash, 并且有改动,就更新翻译缓存
      if let Some(map) = traned_src.map.get_mut(path)
        && let Some((from_lang, src_hash)) = map.remove(&lang)
      {
        update_li
          .entry((from_lang, lang))
          .or_default()
          .push(tran_api::UpdateFile {
            txt: rstr(path)?,
            src_hash,
            path: path.clone(),
          });
      }

      // 如果是需要翻译的源文件, 并且路径匹配, 就添加到 tran
      if let Some(to_li) = tran_glob.get(lang, path) {
        tran_li_push(path, to_li);
      }
    }
  }

  dbg!(&update_li);
  Ok(ScanResult {
    update_li: update_li
      .into_iter()
      .map(|((from_lang, to_lang), li)| tran_api::Update {
        from_lang,
        to_lang,
        li,
      })
      .collect(),
    tran_li,
    save: Save {
      root,
      dir_i18n,
      traned_src,
      lang_diff,
      term_diff,
      term_pre_now,
    },
  })
}
