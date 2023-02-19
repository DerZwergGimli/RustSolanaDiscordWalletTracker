#[cfg(test)]
mod parse_test {
    use std::str::FromStr;
    use serde::Serialize;
    use solana_client::rpc_client::RpcClient;
    use solana_sdk::signature::Signature;
    use solana_transaction_status::UiTransactionEncoding;
    use crate::config::config;
    use crate::solana;
    use crate::solana::wallet::Wallet;

    const RPC: &str = "https://try-rpc.mainnet.solana.blockdaemon.tech";

    #[test]
    fn negative_47_ammo() {
        let client = RpcClient::new(RPC);
        let signature = Signature::from_str("VbSsJnDQHTXFsjA65TFbodwUbkLfYJ4TQ5uZEbY7aSvAhxFKxukggmDCY447fMT7HPSig48DL39AivqXzaUXV9B").unwrap();

        let transaction = client.get_transaction(
            &signature,
            UiTransactionEncoding::Json).unwrap();


        let balance_change = Wallet::parse_balance_change(
            transaction,
            "756pfnvP3HHRx1BPwBPQwe1xBMfMWef5N9oN61Ews7np".to_string(),
            "ammoK8AkX2wnebQb35cDAZtTkvsXQbi82cGeTnUvvfK".to_string());


        assert_eq!(balance_change.round(), -47.0);
    }

    #[test]
    fn negative_68_fuel() {
        let client = RpcClient::new("https://api.mainnet-beta.solana.com");
        let signature = Signature::from_str("VbSsJnDQHTXFsjA65TFbodwUbkLfYJ4TQ5uZEbY7aSvAhxFKxukggmDCY447fMT7HPSig48DL39AivqXzaUXV9B").unwrap();

        let transaction = client.get_transaction(
            &signature,
            UiTransactionEncoding::Json).unwrap();


        let balance_change = Wallet::parse_balance_change(
            transaction,
            "756pfnvP3HHRx1BPwBPQwe1xBMfMWef5N9oN61Ews7np".to_string(),
            "fueL3hBZjLLLJHiFH9cqZoozTG3XQZ53diwFPwbzNim".to_string());


        assert_eq!(balance_change.round(), -68.0);
    }

    #[test]
    fn negative_42_food() {
        let client = RpcClient::new(RPC);
        let signature = Signature::from_str("VbSsJnDQHTXFsjA65TFbodwUbkLfYJ4TQ5uZEbY7aSvAhxFKxukggmDCY447fMT7HPSig48DL39AivqXzaUXV9B").unwrap();

        let transaction = client.get_transaction(
            &signature,
            UiTransactionEncoding::Json).unwrap();


        let balance_change = Wallet::parse_balance_change(
            transaction,
            "756pfnvP3HHRx1BPwBPQwe1xBMfMWef5N9oN61Ews7np".to_string(),
            "foodQJAztMzX1DKpLaiounNe2BDMds5RNuPC6jsNrDG".to_string());


        assert_eq!(balance_change.round(), -42.0);
    }

    #[test]
    fn negative_66_tool() {
        let client = RpcClient::new(RPC);
        let signature = Signature::from_str("VbSsJnDQHTXFsjA65TFbodwUbkLfYJ4TQ5uZEbY7aSvAhxFKxukggmDCY447fMT7HPSig48DL39AivqXzaUXV9B").unwrap();

        let transaction = client.get_transaction(
            &signature,
            UiTransactionEncoding::Json).unwrap();


        let balance_change = Wallet::parse_balance_change(
            transaction,
            "756pfnvP3HHRx1BPwBPQwe1xBMfMWef5N9oN61Ews7np".to_string(),
            "tooLsNYLiVqzg8o4m3L2Uetbn62mvMWRqkog6PQeYKL".to_string());


        assert_eq!(balance_change.round(), -66.0);
    }

    #[test]
    fn positive_500_usdc() {
        let client = RpcClient::new(RPC);
        let signature = Signature::from_str("2uhBEJrikpiDWrMszpi8b2k5cknaCqFrDeVQz1pyzbaq7RyT6rdsvxykJCGSznL4GD1CJpu194xJw4TNMQSupqLA").unwrap();

        let transaction = client.get_transaction(
            &signature,
            UiTransactionEncoding::Json).unwrap();


        let balance_change = Wallet::parse_balance_change(
            transaction,
            "756pfnvP3HHRx1BPwBPQwe1xBMfMWef5N9oN61Ews7np".to_string(),
            "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string());


        assert_eq!(balance_change.round(), 500.0);
    }

    #[test]
    fn positive_20_usdc() {
        let client = RpcClient::new(RPC);
        let signature = Signature::from_str("sQ21LGqpG2MirKFXpYpFSJBfjthCLy5pZq2mvtqrWkm6nrmBidZLg7WyCzsueYT1kJxoyLTjGXFu1zBa7sTaXk8").unwrap();

        let transaction = client.get_transaction(
            &signature,
            UiTransactionEncoding::Json).unwrap();


        let balance_change = Wallet::parse_balance_change(
            transaction,
            "756pfnvP3HHRx1BPwBPQwe1xBMfMWef5N9oN61Ews7np".to_string(),
            "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string());

        assert_eq!(balance_change.round(), 20.0);
    }
}