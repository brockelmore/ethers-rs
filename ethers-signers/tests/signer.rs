use ethers::{
    providers::{Http, Provider},
    signers::Wallet,
    types::TransactionRequest,
};
use std::{convert::TryFrom, time::Duration};

#[cfg(not(feature = "celo"))]
mod eth_tests {
    use super::*;
    use ethers::{
        types::BlockNumber,
        utils::{parse_ether, Ganache},
    };

    #[tokio::test]
    async fn pending_txs_with_confirmations_rinkeby_infura() {
        let provider = Provider::<Http>::try_from(
            "https://rinkeby.infura.io/v3/c60b0bb42f8a4c6481ecd229eddaca27",
        )
        .unwrap()
        .interval(Duration::from_millis(2000u64));

        // pls do not drain this key :)
        // note: this works even if there's no EIP-155 configured!
        let client = "FF7F80C6E9941865266ED1F481263D780169F1D98269C51167D20C630A5FDC8A"
            .parse::<Wallet>()
            .unwrap()
            .connect(provider);

        let tx = TransactionRequest::pay(client.address(), parse_ether(1u64).unwrap());
        let tx_hash = client
            .send_transaction(tx, Some(BlockNumber::Pending))
            .await
            .unwrap();
        let receipt = client
            .pending_transaction(tx_hash)
            .confirmations(3)
            .await
            .unwrap();

        // got the correct receipt
        assert_eq!(receipt.transaction_hash, tx_hash);
    }

    #[tokio::test]
    async fn send_eth() {
        let ganache = Ganache::new().spawn();

        // this private key belongs to the above mnemonic
        let wallet: Wallet = ganache.keys()[0].clone().into();
        let wallet2: Wallet = ganache.keys()[1].clone().into();

        // connect to the network
        let provider = Provider::<Http>::try_from(ganache.endpoint())
            .unwrap()
            .interval(Duration::from_millis(10u64));

        // connect the wallet to the provider
        let client = wallet.connect(provider);

        // craft the transaction
        let tx = TransactionRequest::new().to(wallet2.address()).value(10000);

        let balance_before = client.get_balance(client.address(), None).await.unwrap();

        // send it!
        client.send_transaction(tx, None).await.unwrap();

        let balance_after = client.get_balance(client.address(), None).await.unwrap();

        assert!(balance_before > balance_after);
    }
}

#[cfg(feature = "celo")]
mod celo_tests {
    use super::*;

    #[tokio::test]
    async fn test_send_transaction() {
        // Celo testnet
        let provider = Provider::<Http>::try_from("https://alfajores-forno.celo-testnet.org")
            .unwrap()
            .interval(Duration::from_millis(3000u64));

        // Funded with https://celo.org/developers/faucet
        // Please do not drain this account :)
        let client = "d652abb81e8c686edba621a895531b1f291289b63b5ef09a94f686a5ecdd5db1"
            .parse::<Wallet>()
            .unwrap()
            .connect(provider);

        let balance_before = client.get_balance(client.address(), None).await.unwrap();
        let tx = TransactionRequest::pay(client.address(), 100);
        let tx_hash = client.send_transaction(tx, None).await.unwrap();
        let _receipt = client
            .pending_transaction(tx_hash)
            .confirmations(3)
            .await
            .unwrap();
        let balance_after = client.get_balance(client.address(), None).await.unwrap();
        assert!(balance_before > balance_after);
    }
}
