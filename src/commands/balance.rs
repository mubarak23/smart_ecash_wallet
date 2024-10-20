use std::collections::BTreeMap;
use anyhow::Result;
use cdk::mint_url::MintUrl;
use cdk::nuts::CurrencyUnit;
use cdk::wallet::multi_mint_wallet::MultiMintWallet;
use cdk::Amount;


pub async fn balance(multi_mint_wallet: &MultiMintWallet) -> Result<()> {
    mint_balances(multi_mint_wallet).await;

    Ok(())
}

pub async fn mint_balances(multi_mint_wallet: &MultiMintWallet) -> Result<Vec<(MintUrl, Amount)>> {
   
  // let wallets = BTreeMap::<MintUrl, Amount> = multi_mint_wallet.get_balances(&CurrencyUnit::Sat).await?;
   let wallets: BTreeMap<MintUrl, Amount> =
        multi_mint_wallet.get_balances(&CurrencyUnit::Sat).await?;

  let mut wallet_vec = Vec::new();

  for (mint_url, amount) in wallets.iter().enumerate(){
    let mint_url = mint_url.clone();
    wallet_vec.push((mint_url, *amount))
  }
    Ok(wallet_vec)
}