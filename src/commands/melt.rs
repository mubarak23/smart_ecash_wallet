use std::io;
use std::io::Write;
use std::str::FromStr;

use anyhow::{bail, Result};
use cdk::nuts::CurrencyUnit;
use cdk::wallet::multi_mint_wallet::{MultiMintWallet, WalletKey};
use cdk::Bolt11Invoice;
use clap::arg;

use crate::commands::balance::mint_balances;

#[derive(Args)]
pub struct MeltCommand {
  #[arg(default_value = "sat")]
  unit: String
}

pub async fn pay(
  command_args: &MeltCommand,
  multi_mint_wallet: &MultiMintWallet,
) -> Result<()> {
  let unit = CurrencyUnit::from_str(&command_args.unit)?;
  let mints_amounts = mint_balances(multi_mint_wallet).await?;

  println!("Enter mint number to melt from");

  let mut user_input = String::new();
  let stdin = io::stdin();
  stdin.read_line(&mut user_input);

  let mint_number: usize = user_input.trim().parse()?;
  
  if mint_number.gt(&(mint_number.len() - 1)) {
    bail!("Invalid mint number");
  }

  let wallet = mints_amounts[mint_number].0.clone();

  let wallet = multi_mint_wallet.get_wallet(&WalletKey::new(wallet, unit)).await.expect("Known Wallet");

    println!("Enter bolt11 Invoice Request");
  let mut user_input = String::new();
  let stdin = io::stdin();
  io::stdout().flush().unwrap();
  stdin.read_line(&mut user_input);

  let bolt11 = Bolt11Invoice::from_str(user_input.trim());

  if bolt11.amount_milli_satoshis().unwrap().gt(&(<cdk::Amount as Into<u64>>::into(mints_amounts[mint_number].1) * 1000_u64)){
    bail!("Not enough funds");
  }

  // Let melt 
  let melt_quote = wallet.melt_quote(bolt11.to_string(), None).await?;

  let melt = wallet.ment(&melt_quote.id).await?;

  println!("Paid invoice: {}", melt.state);

  if let Some(preimage) = melt.preimage {
    println!("Payment preimage: {}", preimage);
  }

  Ok(())

}