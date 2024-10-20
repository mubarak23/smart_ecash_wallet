use std::{fs, sync::Arc};
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


// WALLET CLI APPLICATION FOR INTERACTING WITH CASHUE
#[derive(Parser)]
#[command(name = "cashu-cli-wallet")]
#[command(author = "mubarak23 <mubarakaminu340@gmail.com>")]
#[command(version = "0.1.0")]
#[command(author, version, about, long_about = None)]

struct cli {
    #[command(subcommand)]
    command: commands
}

#[derive(Subcommand)]
enum Commands {
    Mint(commands::mint::MintCommand),
    Send(commands::Send::SendCommand),
    Balance,
    Recieve(commands::Recieve::RecieveCommand),

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

    let evn_filter = EnvFilter::new(format!("{}, {}", default_filter, sqlx_filter));
    
    // parse user input
    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    fs::create_dir_all(&DEFAULT_WORK_DIR);

    let localstore = create_localstore().await?;

    let mut rug = rand::thread_rng();
    let random_bytes: [u8; 32] = rng.gen();
    let mnemonics = Mnemonic::from_entropy(&random_bytes);

    let multi_mint_wallet = create_multimint_wallet(&mnemonics.to_seed_normalized(""), localstore.clone()).await?;

    match args.command {
        Commands::Mint(command_args) => {
            commands::mint::mint(
                &mnemonics.to_seed_normalized(""),
                &multi_mint_wallet,
                localstore,
                &command_args
            ).await?
        },
         Commands::ListProof(command_args) => {
            commands::list_proofs::list_proofs(
                &mnemonics.to_seed_normalized(""),
                &multi_mint_wallet,
                localstore,
                &command_args
            ).await?
        },
        Commands::Send(command_args) => {
            commands::send::send(&multi_mint_wallet, &command_args).await?
        },
        Commands::Recieve(command_args) => {
            commands::receive::receive(&multi_mint_wallet, &command_args).await?
        },
         Commands::Pay(command_args) => {
            commands::melt::pay(&multi_mint_wallet, &command_args).await?
        },
        Commands::DecodeToken(command_args) => {
            commands::decode_token::decode_token(&command_args).await?
        },
         Commands::Balance(command_args) => {
            commands::balance::balance(&multi_mint_wallet).await?
        },

    }
    Ok(())
}

async fn create_localstore() -> Arc<dyn WalletDatabase<Err = cdk_database::Error> + Send + Sync> {
    todo!()
}

// fn main() {
//     println!("Hello, world!");
// }
