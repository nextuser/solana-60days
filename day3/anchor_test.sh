 
unset ANCHOR_BUILD_DIR 
unset CARGO_TARGET_DIR

anchor test --provider.wallet `solana config get keypair | awk -F ':' '{print $2}' ` --skip-local-validator
