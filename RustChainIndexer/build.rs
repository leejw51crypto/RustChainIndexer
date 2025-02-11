use std::fs;
fn main() -> Result<(), Box<dyn std::error::Error>> {                                                 
    let config = tonic_build::configure().out_dir("src/proto");                                             
    let mut src = Vec::<&str>::new();

    src.push("proto/cosmos/auth/v1beta1/auth.proto");
    src.push("proto/cosmos/auth/v1beta1/genesis.proto");
    src.push("proto/cosmos/auth/v1beta1/query.proto");
    src.push("proto/cosmos/staking/v1beta1/staking.proto");
    src.push("proto/cosmos/staking/v1beta1/genesis.proto");
    src.push("proto/cosmos/staking/v1beta1/query.proto");
    src.push("proto/cosmos/staking/v1beta1/tx.proto");
    src.push("proto/cosmos/bank/v1beta1/bank.proto");
    src.push("proto/cosmos/bank/v1beta1/genesis.proto");
    src.push("proto/cosmos/bank/v1beta1/query.proto");
    src.push("proto/cosmos/bank/v1beta1/tx.proto");
    src.push("proto/cosmos/vesting/v1beta1/vesting.proto");
    src.push("proto/cosmos/vesting/v1beta1/tx.proto");
    src.push("proto/cosmos/tx/signing/v1beta1/signing.proto");
    //src.push("proto/cosmos/tx/v1beta1/service.proto");
    src.push("proto/cosmos/tx/v1beta1/tx.proto");

    src.push("proto/cosmos/base/reflection/v1beta1/reflection.proto");

    src.push("proto/cosmos/slashing/v1beta1/slashing.proto");
    src.push("proto/cosmos/slashing/v1beta1/genesis.proto");
    src.push("proto/cosmos/slashing/v1beta1/query.proto");
    src.push("proto/cosmos/slashing/v1beta1/tx.proto");

    src.push("proto/cosmos/distribution/v1beta1/distribution.proto");
    src.push("proto/cosmos/distribution/v1beta1/genesis.proto");
    src.push("proto/cosmos/distribution/v1beta1/query.proto");
    src.push("proto/cosmos/distribution/v1beta1/tx.proto");


    src.push("proto/cosmos/mint/v1beta1/genesis.proto");
    src.push("proto/cosmos/mint/v1beta1/mint.proto");
    src.push("proto/cosmos/mint/v1beta1/query.proto");
    src.push("proto/cosmos/gov/v1beta1/gov.proto");
    src.push("proto/cosmos/gov/v1beta1/genesis.proto");
    src.push("proto/cosmos/gov/v1beta1/query.proto");
    src.push("proto/cosmos/gov/v1beta1/tx.proto");


    src.push("proto/cosmos/upgrade/v1beta1/upgrade.proto");
    src.push("proto/cosmos/upgrade/v1beta1/query.proto");

    src.push("proto/tendermint/types/types.proto");
    src.push("proto/tendermint/types/validator.proto");
    src.push("proto/tendermint/types/evidence.proto");
    src.push("proto/tendermint/types/params.proto");
    src.push("proto/tendermint/types/block.proto");
    src.push("proto/tendermint/types/events.proto");
    src.push("proto/tendermint/blockchain/types.proto");

    src.push("proto/cosmos/mainservice.proto");
    config.compile(&src[..],&["proto/"])?;                                                

/*
    fs::rename("src/proto/tendermint.types.rs", "src/proto/tendermint_types.rs");
    fs::rename("src/proto/tendermint.version.rs", "src/proto/tendermint_version.rs");
    fs::rename("src/proto/tendermint.blockchain.rs", "src/proto/tendermint_blockchain.rs");
    fs::rename("src/proto/tendermint.crypto.rs", "src/proto/tendermint_crypto.rs");
    fs::rename("src/proto/tendermint.abci.rs", "src/proto/tendermint_abci.rs");

    fs::rename("src/proto/cosmos.bank.v1beta1.rs", "src/proto/cosmos_bank_v1beta1.rs");
    fs::rename("src/proto/cosmos.auth.v1beta1.rs", "src/proto/cosmos_auth_v1beta1.rs");
    fs::rename("src/proto/cosmos.staking.v1beta1.rs", "src/proto/cosmos_staking_v1beta1.rs");
    fs::rename("src/proto/cosmos.slashing.v1beta1.rs", "src/proto/cosmos_slashing_v1beta1.rs");
    fs::rename("src/proto/cosmos.tx.v1beta1.rs", "src/proto/cosmos_tx_v1beta1.rs");
    fs::rename("src/proto/cosmos.tx.signing.v1beta1.rs", "src/proto/cosmos_tx_signing_v1beta1.rs");


    fs::rename("src/proto/cosmos.gov.v1beta1.rs", "src/proto/cosmos_gov_v1beta1.rs");
    fs::rename("src/proto/cosmos.mint.v1beta1.rs", "src/proto/cosmos_mint_v1beta1.rs");
    fs::rename("src/proto/cosmos.upgrade.v1beta1.rs", "src/proto/cosmos_upgrade_v1beta1.rs");
    fs::rename("src/proto/cosmos.vesting.v1beta1.rs", "src/proto/cosmos_vesting_v1beta1.rs");
    fs::rename("src/proto/cosmos.distribution.v1beta1.rs", "src/proto/cosmos_distribution_v1beta1.rs");
    */

    Ok(())                                                                                            
} 
