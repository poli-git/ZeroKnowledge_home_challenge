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
use url::Url;


use opentelemetry::{
    global, runtime,
    sdk::{propagation::TraceContextPropagator, trace, Resource},
    trace::{TraceContextExt, TraceError, Tracer},
    Key, KeyValue,
};
use opentelemetry_otlp::WithExportConfig;
use rand::Rng;

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


// create a constant key
const RANDOM: Key = Key::from_static_str("random.value");

fn init_tracer() -> Result<trace::Tracer, TraceError> {
    // Initialise OTLP Pipeline
    opentelemetry_otlp::new_pipeline()
        .tracing() // create OTLP tracing pipeline
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic() // create GRPC layer 
                .with_endpoint("http://host.docker.internal:4317"), // GRPC OTLP Jaeger Endpoint
        )
        // Trace provider configuration 
        .with_trace_config(
            trace::config().with_resource(Resource::new(vec![KeyValue::new(
                opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                "rust-otlp-basic",
            )])),
        )
        .install_batch(runtime::Tokio) // configure a span exporter
}



fn gen_number() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    // Parse CLI Arguments: The application starts by parsing command-line arguments provided by the user.
    let args = Args::parse();


    //

     // set the Global Propagator
     global::set_text_map_propagator(TraceContextPropagator::new());

     // intialise the tracer
     let tracer = init_tracer()?;
 
     // start a new active span
     tracer.in_span("PEPEPEP generating number", |cx| {
         let span = cx.span();
         let num = gen_number();
         span.add_event(
             "FIFUFUUUU opentel demo event Generating Number".to_string(),
             vec![Key::new("number").i64(num.into())],
         );
 
         // set an active span attribute
         span.set_attribute(RANDOM.i64(10));
 
 
         // start a new span
         tracer.in_span("PAPAPA generate another number", |cx| {
             let span = cx.span();
             let num = gen_number();
             span.add_event(
                 "ZEZEZE Generating Number".to_string(),
                 vec![Key::new("number").i64(num.into())],
             )
         })
     });
 
     // gracefully shutdown the tracer
     global::shutdown_tracer_provider();


    //


    // Create an alloy provider for that private key and URL.
    let wallet = EthereumWallet::from(args.eth_wallet_private_key);
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_http(args.rpc_url);

    // ABI encode input: Before sending the proof request to the Bonsai proving service,
    // the input number is ABI-encoded to match the format expected by the guest code running in the zkVM.
    let input = args.input.abi_encode();

    let env = ExecutorEnv::builder().write_slice(&input).build()?;

    let receipt = default_prover()
        .prove_with_ctx(
            env,
            &VerifierContext::default(),
            IS_ODD_ELF,
            &ProverOpts::groth16(),
        )?
        .receipt;

    // Encode the seal with the selector.
    let seal = encode_seal(&receipt)?;

    // Extract the journal from the receipt.
    let journal = receipt.journal.bytes.clone();

    // Decode Journal: Upon receiving the proof, the application decodes the journal to extract
    // the verified number. This ensures that the number being submitted to the blockchain matches
    // the number that was verified off-chain.
    let x = U256::abi_decode(&journal, true).context("decoding journal data")?;

    // Construct function call: Using the IOddNumber interface, the application constructs
    // the ABI-encoded function call for the set function of the OddNumber contract.
    // This call includes the verified number, the post-state digest, and the seal (proof).
    let contract = IOddNumber::new(args.contract, provider);
    let call_builder = contract.set(x, seal.into());

    // Initialize the async runtime environment to handle the transaction sending.
    let runtime = tokio::runtime::Runtime::new()?;

    // Send transaction: Finally, send the transaction to the Ethereum blockchain,
    // effectively calling the set function of the OddNumber contract with the verified number and proof.
    let pending_tx = runtime.block_on(call_builder.send())?;
    runtime.block_on(pending_tx.get_receipt())?;

    Ok(())
}
