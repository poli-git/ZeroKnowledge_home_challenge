mod trace_logs;
use alloy::{
    network::EthereumWallet, providers::ProviderBuilder, signers::local::PrivateKeySigner,
    sol_types::SolValue,
};
use alloy_primitives::{Address, U256};
use anyhow::{Context, Result};
use clap::Parser;
use methods::IS_ODD_ELF;
use risc0_ethereum_contracts::encode_seal;
use risc0_zkvm::{default_prover, ExecutorEnv, ProverOpts, VerifierContext};
use trace_logs::init_tracing;
use url::Url;

// `IOddNumber` interface automatically generated via the alloy `sol!` macro.
alloy::sol!(
    #[sol(rpc, all_derives)]
    "../contracts/IOddNumber.sol"
);

/// Arguments of the publisher CLI.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Ethereum chain ID
    #[clap(long)]
    chain_id: u64,

    /// Ethereum Node endpoint.
    #[clap(long, env)]
    eth_wallet_private_key: PrivateKeySigner,

    /// Ethereum Node endpoint.
    #[clap(long)]
    rpc_url: Url,

    /// Application's contract address on Ethereum
    #[clap(long)]
    contract: Address,

    /// The input to provide to the guest binary
    #[clap(short, long)]
    input: U256,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    init_tracing();

    tracing::info!("Starting process...");

    // Parse CLI Arguments: The application starts by parsing command-line arguments provided by the user.
    let args = Args::parse();

    // Create an alloy provider for that private key and URL.
    let wallet = EthereumWallet::from(args.eth_wallet_private_key);
    let provider = ProviderBuilder::new().wallet(wallet).on_http(args.rpc_url);
    tracing::debug!("Created alloy provider");

    // ABI encode input: Before sending the proof request to the Bonsai proving service,
    // the input number is ABI-encoded to match the format expected by the guest code running in the zkVM.
    let input = args.input.abi_encode();
    tracing::debug!("Input has been encoded");

    let env = ExecutorEnv::builder().write_slice(&input).build()?;

    let receipt = default_prover()
        .prove_with_ctx(
            env,
            &VerifierContext::default(),
            IS_ODD_ELF,
            &ProverOpts::groth16(),
        )?
        .receipt;
    tracing::debug!("Got Receipt of the zero-knowledge proof of computation");

    // Encode the seal with the selector.
    let seal = encode_seal(&receipt)?;

    // Extract the journal from the receipt.
    let journal = receipt.journal.bytes.clone();

    // Decode Journal: Upon receiving the proof, the application decodes the journal to extract
    // the verified number. This ensures that the number being submitted to the blockchain matches
    // the number that was verified off-chain.
    let x = U256::abi_decode(&journal, true).context("decoding journal data")?;
    tracing::debug!("Journal data decoded");

    // Construct function call: Using the IOddNumber interface, the application constructs
    // the ABI-encoded function call for the set function of the OddNumber contract.
    // This call includes the verified number, the post-state digest, and the seal (proof).
    let contract = IOddNumber::new(args.contract, provider);
    let call_builder = contract.set(x, seal.into());

    // Send transaction: Finally, send the transaction to the Ethereum blockchain,
    // effectively calling the set function of the OddNumber contract with the verified number and proof.
    let pending_tx = call_builder.send().await?;

    // Wait for the transaction to be included and get the receipt.
    let receipt = pending_tx.get_receipt().await?;

    tracing::info!("Process completed - Proof has been published successfully - TX Hash: {:?}", receipt.transaction_hash.to_string());

    Ok(())
}
