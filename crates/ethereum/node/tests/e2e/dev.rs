use crate::utils::EthNode;
use futures::StreamExt;
use reth::rpc::eth::EthTransactions;
use reth_chainspec::ChainSpec;
use reth_e2e_test_utils::setup;
use reth_primitives::{b256, hex, Genesis};
use reth_provider::CanonStateSubscriptions;
use std::sync::Arc;

#[tokio::test]
async fn can_run_dev_node() -> eyre::Result<()> {
    reth_tracing::init_test_tracing();
    let (mut nodes, _tasks, _) = setup(1, custom_chain(), true).await?;

    assert_chain_advances(nodes.pop().unwrap()).await;
    Ok(())
}

async fn assert_chain_advances(mut node: EthNode) {
    let mut notifications = node.inner.provider.canonical_state_stream();

    // submit tx through rpc
    let raw_tx = hex!("02f876820a28808477359400847735940082520894ab0840c0e43688012c1adb0f5e3fc665188f83d28a029d394a5d630544000080c080a0a044076b7e67b5deecc63f61a8d7913fab86ca365b344b5759d1fe3563b4c39ea019eab979dd000da04dfc72bb0377c092d30fd9e1cab5ae487de49586cc8b0090");

    let eth_api = node.inner.rpc_registry.eth_api();

    let hash = eth_api.send_raw_transaction(raw_tx.into()).await.unwrap();

    let expected = b256!("b1c6512f4fc202c04355fbda66755e0e344b152e633010e8fd75ecec09b63398");

    assert_eq!(hash, expected);
    println!("submitted transaction: {hash}");

    let head = notifications.next().await.unwrap();

    let tx = head.tip().transactions().next().unwrap();
    assert_eq!(tx.hash(), hash);
    println!("mined transaction: {hash}");
}

fn custom_chain() -> Arc<ChainSpec> {
    let custom_genesis = r#"
{

    "nonce": "0x42",
    "timestamp": "0x0",
    "extraData": "0x5343",
    "gasLimit": "0x1388",
    "difficulty": "0x400000000",
    "mixHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
    "coinbase": "0x0000000000000000000000000000000000000000",
    "alloc": {
        "0x6Be02d1d3665660d22FF9624b7BE0551ee1Ac91b": {
            "balance": "0x4a47e3c12448f4ad000000"
        }
    },
    "number": "0x0",
    "gasUsed": "0x0",
    "parentHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
    "config": {
        "ethash": {},
        "chainId": 2600,
        "homesteadBlock": 0,
        "eip150Block": 0,
        "eip155Block": 0,
        "eip158Block": 0,
        "byzantiumBlock": 0,
        "constantinopleBlock": 0,
        "petersburgBlock": 0,
        "istanbulBlock": 0,
        "berlinBlock": 0,
        "londonBlock": 0,
        "terminalTotalDifficulty": 0,
        "terminalTotalDifficultyPassed": true,
        "shanghaiTime": 0
    }
}
"#;
    let genesis: Genesis = serde_json::from_str(custom_genesis).unwrap();
    Arc::new(genesis.into())
}
