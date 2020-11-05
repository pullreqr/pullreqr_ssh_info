use serde::ser::Serialization;
use serde_json;
use std::env;
use pretty_env_logger::{info, error, warn, trace, debug};

#[derive(Serialize)]
enum PullReqProtocol {
  Gerrit,
  AGit,
}

struct SSHInfo {
   fetch: String
   pushurl: String
   proto: PullReqProtocol
}


fn main() {
   let mut log pretty_env_logger::formatted_builder().target(File::create("~/logs/ssh_info.log"));
   for var in env::var() {
      debug!(format!("{} = {}", v
   }
}
