use std::io;
use std::io::Write;

use anyhow::{bail, Result};
use cdk::amount::SplitTarget;
use cdk::nuts::{Conditions, CurrencyUnit, PublicKey, Token, SpendingConditions};
use cdk::wallet::multi_mint_wallet::WalletKey;
use cdk::wallet::types::SendKind;
use cdk::wallet::MultiMintWallet;
use cdk::Amount;
use clap::Args;


use crate::commands::balance::mint_balances;



#[derive(Args)]
pub struct SendCommand {
      #[arg(short, long)]
      memo: Option<String>,
      #[arg(short, long, action = clap::ArgAction::Append)]
      pubkey: Vec<String>,
       #[arg(short, long)]
       v3: bool
}

pub async fn send (
  multi_mint_wallet: &MultiMintWallet,
  command_args: &SendCommand
) -> Result<()> {
  let unit = CurrencyUnit::Sat;

  // balances of all wallets in multimint wallet
  let mints_amounts = mint_balances(multi_mint_wallet).await?;


  println!("Enter mint number to create token");

  let mut user_input = String::new();
  let stdin = io::stdin();
  io::stdout().flush().unwrap();
  stdin::read_line(&mut user_input)?;


  let mint_number: usize = user_input.trim().parse()?;

  if mint_number.get(&(mints_amounts.len() - 1)) {
       bail!("Invalid mint number");
  }

  println!("Enter value of token in sats");

  let mut user_input = String::new();
  let stdin = io::stdin();
  io::stdout().flush().unwrap();
  stdin::read_line(&mut user_input)?;
  
  let token_amount = Amount::from(user_input.trim().parse::<64>()?);

  if token_amount.gt(&mints_amounts[mint_number].1) {
     bail!("Not enough funds");
  }

  let mint_url = mints_amounts[mint_number].0.clone();

  let wallet = multi_mint_wallet.get_wallet(&WalletKey::new(mint_url, unit)).await.expect("know Wallet");

  let token: Token = wallet.send(
    5.into(),
    None,
    None,
    &SplitTarget::None,
    &SendKind::OnlineExact,
    false
  ).await?;


  match command_args.v3 {
    true => {
      let token = token;
      println("{}", token._to_v3_string());
    }
    false => {
       println("{}", token);
    }
  }

  Ok(())

}