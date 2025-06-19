#![feature(let_chains)]

use uper::{ArgMatches, Command};
use aok::{OK, Void};
use clap::arg;
use i18::{Error, tran};

#[static_init::constructor(0)]
extern "C" fn _loginit() {
  loginit::init();
}

fn cmd_build(cmd: Command) -> Command {
  cmd.arg(arg!(-w --workdir [path] "workdir"))
}

async fn run(matches: ArgMatches) -> Void {
  let workdir = m
    .get_one("workdir")
    .map(|s: &String| s.into())
    .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| ".".into()));
  if let Err(e) = tran(&workdir).await {
    if let Some(e) = e.downcast_ref::<Error>() {
      eprintln!("{e}");
      return OK;
    }
    return Err(e);
  }
  OK
}

#[tokio::main]
async fn main() -> Void {
  uper::load!(
    upgrade_host::UPGRADE_HOST,
    crate::upgrade::PK,
    cmd_build,
    run
  )
  .await
}
