 anchor test --provider.wallet `solana config get keypair | awk -F ':' '{print $2}' ` --skip-local-validator
