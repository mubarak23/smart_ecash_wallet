use std::{fs, sync::Arc, path::PathBuf, str::FromStr};
use bip39::Mnemonic;
use cdk::{
    cdk_database::{self, WalletDatabase},
    mint_url::MintUrl,
    nuts::CurrencyUnit,
    wallet::{multi_mint_wallet::WalletKey, MultiMintWallet, Wallet}
}; 

use clap::{Parser, Subcommand};
use rand::Rng;
use tracing_subscriber::EnvFilter;

mod commands;


const DEFAULT_WORK_DIR: &str = ".cdk-cli";


// WALLET CLI APPLICATION FOR INTERACTING WITH CASHUE
#[derive(Parser)]
#[command(name = "cashu-cli-wallet")]
#[command(author = "mubarak23 <mubarakaminu340@gmail.com>")]
#[command(version = "0.1.0")]
#[command(author, version, about, long_about = None)]

struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    Mint(commands::mint::MintCommand),
    Send(commands::send::SendCommand),
    Balance,
    Receive(commands::receive::ReceiveCommand),

    /// Pay bolt11 invoice
    Pay(commands::melt::MeltCommand),
    DecodeToken(commands::decode_token::DecodeTokenCommand),
    ListProof(commands::list_proofs::ListProofsCommand),
}

#[tokio::main]
async fn main () -> anyhow::Result<()> {
    let args: Cli = Cli::parse();
    let default_filter = "warn";
    
    let sqlx_filter = "sqlx-warn";

   //  let evn_filter = EnvFilter::new(format!("{}, {}", default_filter, sqlx_filter));
    let env_filter = EnvFilter::new(format!("{},{}", default_filter, sqlx_filter));
   
    // parse user input
    tracing_subscriber::fmt().with_env_filter(env_filter).init();
    // tracing_subscriber::fmt().with_env_filter(env_filter).init();

    fs::create_dir_all(&DEFAULT_WORK_DIR);

    let localstore = create_localstore().await?;

    let mut rng = rand::thread_rng();
    let random_bytes: [u8; 32] = rng.gen();
    let mnemonic = Mnemonic::from_entropy(&random_bytes)?;

    let multi_mint_wallet = create_multimint_wallet(&mnemonic.to_seed_normalized(""), localstore.clone()).await?;

    match args.command {
        Commands::Mint(command_args) => {
            commands::mint::mint(
                &mnemonic.to_seed_normalized(""),
                &multi_mint_wallet,
                localstore,
                &command_args
            ).await?
        },
         Commands::ListProof(command_args) => {
            commands::list_proofs::list_proofs(
                &mnemonic.to_seed_normalized(""),
                &multi_mint_wallet,
                localstore,
                &command_args
            ).await?
        },
        Commands::Send(command_args) => {
            commands::send::send(&multi_mint_wallet, &command_args).await?
        },
        Commands::Receive(command_args) => {
            commands::receive::receive(&multi_mint_wallet, &command_args).await?
        },
         Commands::Pay(command_args) => {
            commands::melt::pay(&multi_mint_wallet, &command_args).await?
        },
        Commands::DecodeToken(command_args) => {
            commands::decode_token::decode_token(&command_args).await?
        },
         Commands::Balance => commands::balance::balance(&multi_mint_wallet).await?,

    }
    Ok(())
}

async fn create_localstore() -> Arc<dyn WalletDatabase<Err = cdk_database::Error> + Send + Sync> {
        let path = PathBuf::from_str(DEFAULT_WORK_DIR)
        .unwrap()
        .join("cdk-wallet");
    let local = cdk_redb::WalletRedbDatabase::new(&path).unwrap();

    Arc::new(local)

}

async fn create_multimint_wallet(
     seed: &[u8],
    localstore: Arc<dyn WalletDatabase<Err = cdk_database::Error> + Sync + Send>,
)-> anyhow::Result<MultiMintWallet> {
    let mut wallets: Vec<Wallet> = Vec::new();

    let mints = localstore.get_mints().await?;

    for(mint, _) in mints {
        let wallet = Wallet::new(
            &mint.to_string(),
            CurrencyUnit::Sat,
            localstore.clone(),
            seed,
            None
        ).unwrap();

        wallets.push(wallet)
    }

    Ok(MultiMintWallet::new(wallets))
}

pub async fn get_single_mint_wallet(
    multi_mint_wallet: &MultiMintWallet,
    seed: &[u8],
    localstore: Arc<dyn WalletDatabase<Err = cdk_database::Error> + Sync + Send>,
    mint_url: MintUrl,
    unit: CurrencyUnit,
) -> anyhow::Result<Wallet> {
    let wallet = match multi_mint_wallet
        .get_wallet(&WalletKey::new(mint_url.clone(), unit))
        .await
    {
        Some(wallet) => wallet.clone(),
        None => {
            let wallet = Wallet::new(
                &mint_url.to_string(),
                CurrencyUnit::Sat,
                localstore,
                seed,
                None,
            )?;

            multi_mint_wallet.add_wallet(wallet.clone()).await;
            wallet
        }
    };

    Ok(wallet)
}


// fn main() {
//     println!("Hello, world!");
// }
