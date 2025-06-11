use std::{
  collections::{BTreeMap, BTreeSet},
  fs,
  path::PathBuf,
};

use saphyr::{LoadableYamlNode, Mapping, Yaml, YamlEmitter};
use aok::{OK, Result, Void};
use ub64::{b64d, b64e};
use xbin::concat;

/*
  记录翻译文件的源hash
*/

#[derive(Default)]
pub struct Src {
  path: PathBuf,
  // rel to_lang from_lang hash , 用 BTreeMap 保证每次保存的yml顺序是一样的
  map: BTreeMap<String, BTreeMap<u32, (u32, Vec<u8>)>>,
}

pub fn load(path: impl Into<PathBuf>) -> Result<Src> {
  let path = path.into();
  let mut map = BTreeMap::new();
  if path.exists() {
    for i in Yaml::load_from_str(&fs::read_to_string(&path)?)? {
      if let Yaml::Mapping(i) = i {
        for (rel, h) in i {
          if let Some(rel) = rel.as_str()
            && let Yaml::Mapping(h) = h
          {
            let rel = rel.to_owned();
            let map = map.entry(rel).or_insert_with(BTreeMap::new);
            for (from_lang_hash, to_lang_li) in h {
              if let Some(from_lang_hash) = from_lang_hash.as_str()
                && let Some(to_lang_li) = to_lang_li.as_str()
                && let Ok(from_lang_hash) = b64d(from_lang_hash)
                && from_lang_hash.len() >= 2
                && let Ok(to_lang_li) = b64d(to_lang_li)
                && let Ok(to_lang_li) = vb::diffd(to_lang_li)
              {
                let from_lang = u16::from_le_bytes(from_lang_hash[0..2].try_into().unwrap()) as u32;

                let hash = from_lang_hash[2..].to_vec();
                for to_lang in to_lang_li {
                  map.insert(to_lang as u32, (from_lang, hash.clone()));
                }
              }
            }
          }
        }
      }
    }
  }
  Ok(Src { path, map })
}

impl Src {
  pub fn get(&self, rel: impl AsRef<str>, from_lang: u32, to_lang: u32) -> Option<Vec<u8>> {
    if let Some(m) = self.map.get(rel.as_ref())
      && let Some(m) = m.get(&to_lang)
      && m.0 == from_lang
    {
      return Some(m.1.clone());
    }
    None
  }

  pub fn add(&mut self, rel: impl Into<String>, from_lang: u32, from_hash: Vec<u8>, to_lang: u32) {
    self
      .map
      .entry(rel.into())
      .or_default()
      .insert(to_lang, (from_lang, from_hash));
  }

  pub fn save(&self) -> Void {
    let mut from_rel_hash_to_lang_li = Mapping::new();

    // rel from_lang from_hash to_lang_li
    for (rel, map) in &self.map {
      if let Yaml::Mapping(h) = from_rel_hash_to_lang_li
        .entry(Yaml::value_from_cow(rel.into()))
        .or_insert_with(|| Yaml::Mapping(Mapping::new()))
      {
        let mut from_lang_hash_to_lang_li = BTreeMap::new();

        for (to_lang, (from_lang, from_hash)) in map {
          from_lang_hash_to_lang_li
            .entry((*from_lang, from_hash))
            .or_insert_with(BTreeSet::new)
            .insert(*to_lang);
        }

        for ((from_lang, from_hash), to_lang_li) in from_lang_hash_to_lang_li {
          h.insert(
            Yaml::value_from_cow(
              b64e(concat([
                &(from_lang as u16).to_le_bytes()[..],
                &from_hash[..],
              ]))
              .into(),
            ),
            Yaml::value_from_cow(
              b64e(vb::diffe(
                to_lang_li.into_iter().map(|i| i as u64).collect::<Vec<_>>(),
              ))
              .into(),
            ),
          );
        }
      }
    }
    let mut yml = String::new();
    let mut emitter = YamlEmitter::new(&mut yml);
    emitter.dump(&Yaml::Mapping(from_rel_hash_to_lang_li))?;
    fs::write(&self.path, yml)?;
    OK
  }
}
