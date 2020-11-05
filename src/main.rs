use serde::{Serialize};
use serde_json as json;
use std::{ 
  io, env, fs, num
};
use dirs_next as dirs;
//use io::Read;
use log::{info, error, warn, trace, debug};
use simplelog as logger;
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
  #[error("IO Error {0}")]
  IO(#[from] io::Error),
  #[error("Couldn't find home directory")]
  HomeDir,
  // We should probably warn for this
  #[error("Could not set logger")]
  Logging(#[from] log::SetLoggerError), 
  #[error("json serialization: {0}")]
  Json(#[from] serde_json::Error),
}
#[derive(Serialize)]
enum PullReqProtocol {
  Gerrit,
  AGit,
}

#[derive(Serialize)]
struct SSHInfo {
   #[serde(skip_serializing_if = "Option::is_none")]
   user: Option<String>,
   #[serde(skip_serializing_if = "Option::is_none")]
   host: Option<String>,
   #[serde(skip_serializing_if = "Option::is_none")]
   port: Option<u16>,
   #[serde(skip_serializing_if = "Option::is_none")]
   r#type: Option<PullReqProtocol>,
   #[serde(skip_serializing_if = "Option::is_none")]
   fetch: Option<String>,
   #[serde(skip_serializing_if = "Option::is_none")]
   pushurl: Option<String>,
   version: num::NonZeroUsize,
   expire: usize,
}

fn main() -> Result<(), Error> {
   let home = dirs::home_dir().ok_or(Error::HomeDir)?;
   // let mut log_file = io::stderr(); 
   let log_file = fs::File::create(home.join("logs").join("ssh_info.log"))?;
   logger::WriteLogger::init(logger::LevelFilter::Debug, logger::Config::default(), log_file)?;

   for (key, val) in env::vars() {
      debug!("{} = {}", key, val);
   }

   let mut input = String::new();

   debug!("--stdin--\n");
   while let Ok(x) = io::stdin().read_line(&mut input) {
	if x == 0 {
		break;
	}
   	debug!("\t{}", input);
	input.clear()
   }
   debug!("--stdin EOF--\n");
   //echo "{\"user\": \"${USER}\", \"host\": \"localhost\", \"port\": 2222, \"type\": \"agit\", \"version\": 1, \"expire\": 0}"
   let ssh_info = SSHInfo{
	user: Some(whoami::username()),
	host: Some("localhost".to_string()),
        port: Some(2222),
        r#type: Some(PullReqProtocol::AGit),
	fetch: None,
	pushurl: None,
        version: unsafe { num::NonZeroUsize::new_unchecked(1) },
	expire: 0
   };
   println!("{}", json::to_string(&ssh_info)?);
   Ok(())
}
