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
    src.push("proto/cosmos/tx/v1beta1/service.proto");
    src.push("proto/cosmos/tx/v1beta1/tx.proto");

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

    config.compile(&src[..],&["proto/"])?;                                                
    Ok(())                                                                                            
} 
