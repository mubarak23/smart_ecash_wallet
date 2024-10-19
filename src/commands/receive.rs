use std::str::FromStr;

use anyhow::Result;
use cdk::nuts::SecretKey;
use cdk::wallet::multi_mint_wallet::MultiMintWallet;
use clap::Args;


#[derive(Args)]
pub struct RecieveCommand {
  token: String,
  #[arg(short, long, action = clap::ArgAction::Append)]
  signing_key: Vec<String>
}


pub async fn recieve (
  multi_mint_wallet: &MultiMintWallet,
  command_args: &RecieveCommand
) -> Result<()> {
  let signing_key = Vec::new();

  if !sub_command_args.signing_key.is_empty() {
    let mut s_keys: Vec<SecretKey> = sub_command_args
      .signing_key.iter().map(|s| {
        if s.start_with("nsec") {
          let nostr_key = nostr_sdk::SecretKey::from_str(s).expect("Invalid secret key");

          SecretKey::from_str(&nostr_key.to_secret_hex())
        } else {
          SecretKey::from_str(s)
        }
      }).collect::<Result<Vec<SecretKey>, _>>()?
      signing_key.append(&mut s_keys);
  }

  let token_str = &command_args.token;

  let amount = multi_mint_wallet.recieve(&token_str, &[], &[]).await?;

  println!("Amount Recieve: {}", amount)
}