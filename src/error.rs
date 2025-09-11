const TOKEN_TIP: &str = r#":

Get from https://i18n.site/token

Write ~/.config/i18/conf 

token: YOUR_TOKEN

Or set env I18_TOKEN
"#;

#[derive(thiserror::Error, Debug)]
pub enum Error {
  // #[error("{0} : {1}")]
  // Conf(PathBuf, serde_yaml::Error),
  //
  // // #[error("tran error {0} : {1}")]
  // // Tran(u16, String),
  //
  // // 后台返回的错误, 比如: 欠费
  // #[error("api error {code} : {msg}")]
  // Api { code: i32, msg: String },
  #[error("TOKEN MISS {TOKEN_TIP}")]
  Token,

  #[error("miss .i18n/tran.yml")]
  MissTranYml,

  #[error(".i18n/tran.yml miss default from lang")]
  MissDefaultFrom,

  #[error("grpc {url} : {err}")]
  Grpc {
    url: String,
    err: tonic::transport::Error,
  },
}
