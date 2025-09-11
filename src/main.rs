use uper::{ArgMatches, arg};
use aok::{OK, Void};
use i18::{Error, tran};
mod upgrade;
use upgrade::PK;

#[static_init::constructor(0)]
extern "C" fn _log_init() {
  log_init::init();
}

async fn run(matches: ArgMatches) -> Void {
  let workdir = matches
    .get_one("workdir")
    .map(|s: &String| s.into())
    .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| ".".into()));

  let conf = matches
    .get_one("config")
    .map(|s: &String| s.into())
    .unwrap_or_else(|| confdir::confdir().join(env!("CARGO_PKG_NAME")).join("conf"));

  let conf = confer::env_conf!(conf);

  let grpc = conf
    .str("grpc")
    .unwrap_or_else(|| "http://127.0.0.1:3333".into());

  if let Some(token) = conf.str("token") {
    if let Err(err) = tran(&grpc, token, &workdir).await {
      eprintln!("{grpc} : {err}");
      std::process::exit(1);
    }
  } else {
    return Err(crate::Error::Token.into());
  }

  OK
}

#[tokio::main]
async fn main() -> Void {
  // uper 负责自动更新
  uper::load!(
    upgrade_host::UPGRADE_HOST,
    PK,
    |cmd| {
      cmd
        .arg(arg!(-w --workdir [path] "workdir"))
        .arg(arg!(-c --config [path] "config"))
    },
    async |matches| {
      if let Err(err) = run(matches).await {
        if let Some(err) = err.downcast_ref::<Error>() {
          eprintln!("{err}");
          std::process::exit(1);
        }
        return Err(err);
      }
      OK
    }
  )
}
