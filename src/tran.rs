use std::{
  collections::{HashMap, hash_map::Entry},
  io::Write,
  path::Path,
};

use tran_api::{Job, tran_result::Kind};
use lang::CODE;
use tokio_stream::StreamExt;
use aok::{OK, Void};
use log::error;

use crate::{scan, scan_result::ScanResult};

fn lang_name(lang: u32) -> String {
  let lang = lang as usize;
  if lang < CODE.len() {
    CODE[lang].into()
  } else {
    lang.to_string()
  }
}

pub async fn tran(grpc: impl Into<String>, token: impl AsRef<str>, root: &Path) -> Void {
  let grpc = grpc.into();
  let ScanResult {
    mut save,
    update_li,
    tran_li,
  } = scan(root)?;

  let mut from_lang_count = HashMap::new();
  for i in &tran_li {
    *from_lang_count.entry(i.from_lang).or_insert(0u64) += 1;
  }
  let token = token.as_ref();

  let api = tran_api::conn(grpc.clone(), |mut req: tonic::Request<()>| {
    req.metadata_mut().insert(
      "t",
      token
        .try_into()
        .map_err(|_| tonic::Status::invalid_argument("invalid token"))?,
    );
    Ok(req)
  })
  .await
  .map_err(|err| crate::Error::Grpc {
    url: grpc.clone(),
    err,
  })?;

  let mut stream = api
    .send_compressed(tonic::codec::CompressionEncoding::Zstd)
    .accept_compressed(tonic::codec::CompressionEncoding::Zstd)
    .tran(Job { tran_li, update_li })
    .await?
    .into_inner();

  while let Some(result) = stream.next().await {
    match result {
      Ok(tran_result) => {
        if let Some(kind) = tran_result.kind {
          match kind {
            Kind::Traned(traned) => {
              let to_lang = traned.to_lang as usize;
              if to_lang < CODE.len() {
                let code = CODE[to_lang];
                {
                  let mut f = ifs::w(root.join(code).join(&traned.path))?;
                  f.write_all(traned.txt.as_bytes())?;
                }
                println!(" ✅ {code}/{}", traned.path);
                save.refresh(to_lang, &traned.path)?;
                let from_lang = traned.from_lang;

                save
                  .traned_src
                  .add(traned.path, from_lang, traned.src_hash, to_lang as _);

                if let Entry::Occupied(mut entry) = from_lang_count.entry(from_lang) {
                  let val = *entry.get();
                  if val > 1 {
                    *entry.get_mut() = val - 1;
                  } else {
                    save.save_lang(from_lang);
                    entry.remove();
                  }
                }
              } else {
                error!("❌ miss lang code {} , please upgrade cli", to_lang);
              }
            }
            Kind::ErrTran(err) => {
              error!(
                "❌ Tran Error: {} → {} {}: {}",
                lang_name(err.from_lang),
                lang_name(err.to_lang),
                err.path,
                err.msg
              );
            }
            Kind::ErrTokenInvalid(_) => {
              error!("❌ Invalid Token");
            }
            Kind::ErrOverdraw(err) => {
              error!("❌ overdraw debt={} currency={}", err.debt, err.currency);
            }
            Kind::ErrUnsupportExt(err) => {
              error!(
                "❌ Unsupported Filetype: {}/{}",
                lang_name(err.from_lang),
                err.path
              );
            }
            Kind::ErrParse(err) => {
              error!(
                "❌ Parse Error: {}/{}: {}",
                lang_name(err.from_lang),
                err.path,
                err.msg
              );
            }
            Kind::ErrUpdate(err) => {
              let kind_str = match tran_api::ErrUpdateKind::try_from(err.kind) {
                Ok(k) => format!("{k:?}"),
                Err(_) => "unknown".to_string(),
              };
              error!(
                "❌ Update Error: {} ← {} {}: {}",
                lang_name(err.from_lang),
                lang_name(err.to_lang),
                err.path,
                kind_str
              );
            }
          }
        }
      }
      Err(err) => {
        error!("{err}");
        return Err(err.into());
      }
    }
  }

  save.save()?;

  OK
}
