use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use cdk::amount::SplitTarget;
use cdk::cdk_database::{Error, WalletDatabase};
use cdk::mint_url::MintUrl;
use cdk::nuts::{MintQuoteState, CurrencyUnit};
use cdk::wallet::MultiMintWallet;
use cdk::Amount;
use clap::Args;
use serde::{Serialize, Deserialize};
use tokio::time::sleep;

use crate::get_single_mint_wallet;


#[derive(Args, Serialize, Deserialize)]
pub struct MintCommand {
  mint_url: MintUrl,
  amount: u64
}


pub async fn mint(
  seed: &[u8],
  multi_mint_wallet: &MultiMintWallet,
   localstore: Arc<dyn WalletDatabase<Err = Error> + Sync + Send>,
   command_args: &MintCommand
) -> Result<()> {
  let mint_url = command_args.mint_url.clone();

  let wallet = get_single_mint_wallet(
    multi_mint_wallet,
    seed,
    localstore,
    mint_url.clone,
    CurrencyUnit::Sat
  ).await?;


  let mint_quote = wallet.mint_quote(10.into(), None).await?;

  loop {
    let state = wallet.mint_quote_state(mint_quote.id).await?;

    if state.state = MintQouteState::Paid {
      break;
    }

  }

  // Mint once quote has been paid
  let mint = wallet.mint(&mint_quote.id, SplitTarget::None).await?;

  println!("Mint is: {}", mint)
}
