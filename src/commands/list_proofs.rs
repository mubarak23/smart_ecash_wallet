use std::str::FromStr;
use anyhow::Result;
use cdk::cdk_database::{WalletDatabase, Error};
use cdk::nuts::{CurrencyUnit, MintQuoteState};
use cdk::mint_url::MintUrl;
use cdk::wallet::MultiMintWallet;
use serde::{ Deserialize, Serialize};
use clap::arg;

use crate::get_single_mint_wallet;


#[derive(Args, Serialize, Deserialize)]
pub struct ListProofsCommand {
  mint_url: MintUrl
}


pub async fn list_proofs(
  seed: &[u8],
  multi_mint_wallet: &MultiMintWallet,
  localstore: Arc<dyn WalletDatabase<Err = Error> + Sync + Send>,
  command_args: &ListProofsCommand,
) -> Result<()> {
  let mint_url = command_args.mint_url.clone();


  let wallet = get_single_mint_wallet(
    multi_mint_wallet,
    seed,
    localstore,
    mint_url.clone(),
    CurrencyUnit::Sats
  ).await?;

  let proofs = wallet.get_proofs.await?;

  for proof in proofs {
    println!("Amount: {}, Secret: {}", proof.amount, proof.secret);
  };

  Ok(())
}