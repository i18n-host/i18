use std::{collections::HashSet, path::Path};

use saphyr::{LoadableYamlNode, Yaml, Yaml::Mapping};
use ft::FromTo;
use globset::Glob;
use aok::Result;

use crate::{Error, GlobFromTo};

/// 返回结果的 路径匹配器, 这个路径翻译的语言映射, 和网页路由器很相似
pub fn tran_yml(root: &Path) -> Result<GlobFromTo> {
  let yml = root.join("tran.yml");
  if !yml.exists() {
    Err(Error::MissTranYml)?;
  }

  let mut path_map = vec![];

  for i in Yaml::load_from_str(&ifs::rstr(yml)?)? {
    // 构建每个路径的翻译 from → to , 这是一个 map , 可以有多个语言的映射
    if let Mapping(i) = i {
      for (path, map) in i {
        if let Mapping(map) = map {
          if path.is_null() {
            path_map.push(("*".to_owned(), map));
          } else if let Some(path) = path.into_string() {
            path_map.push((if path.is_empty() { "*".into() } else { path }, map));
          }
        }
      }
    }
  }

  // 确保匹配的路径从长到短
  path_map.sort_by_key(|(p, _)| std::cmp::Reverse(p.len()));

  let mut glob_from_to = vec![];

  for (path, map) in path_map {
    match Glob::new(&path) {
      Ok(glob) => {
        let mut li = vec![];
        let glob = glob.compile_matcher();
        for (from_lang, to_lang_li) in map {
          if let Some(from_lang) = from_lang.as_str() {
            if let Some(from_lang) = lang::by_str(from_lang) {
              let from_lang = from_lang as u32;
              if to_lang_li.is_null() {
                li.push(FromTo {
                  from_lang,
                  to_lang_li: Default::default(),
                })
              } else if let Some(to_lang_li_str) = to_lang_li.as_str() {
                let mut to_lang_li = HashSet::new();
                for i in to_lang_li_str.split_whitespace() {
                  if let Some(to_lang) = lang::by_str(i) {
                    to_lang_li.insert(to_lang as u32);
                  } else {
                    eprintln!(".118n/tran.yml error lang: {i}");
                  }
                }
                if !to_lang_li.is_empty() {
                  let mut to_lang_li: Vec<u32> = to_lang_li.into_iter().collect();
                  to_lang_li.sort();

                  li.push(FromTo {
                    from_lang,
                    to_lang_li,
                  });
                }
              }
            } else {
              eprintln!(".118n/tran.yml error lang: {from_lang}");
            }
          }
        }
        if !li.is_empty() {
          glob_from_to.push((glob, li));
        }
      }
      Err(e) => {
        eprintln!(".118n/tran.yml {path}: {e}");
        continue;
      }
    }
  }
  Ok(GlobFromTo(glob_from_to))
}
