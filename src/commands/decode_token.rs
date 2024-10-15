use std::str::FromStr;
use cdk::nuts::Token;
use anyhow::Result;
use cdk::util::serialize_to_cbor_diag;
use clap::arg;

#[derive(Args)]
pub struct DecodeTokenSubCommand {
  token: String 
}

pub decode_token(command_args: &DecodeTokenSubCommand) -> Result<()> {
  let token = Token::from_str(&command_args.token)?;

  println!("{:}", serialize_to_cbor_diag(&token)?);
}