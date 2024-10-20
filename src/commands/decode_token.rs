use std::str::FromStr;
use cdk::nuts::Token;
use anyhow::Result;
use cdk::util::serialize_to_cbor_diag;
use clap::Args;

#[derive(Args)]
pub struct DecodeTokenCommand {
  token: String 
}

pub fn decode_token(command_args: &DecodeTokenCommand) -> Result<()> {
  let token = Token::from_str(&command_args.token)?;

  println!("{:}", serialize_to_cbor_diag(&token)?);
}