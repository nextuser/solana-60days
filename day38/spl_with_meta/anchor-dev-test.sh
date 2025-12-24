 anchor test --provider.wallet `solana config get keypair | awk -F ':' '{print $2}' `  --provider.cluster https://devnet.helius-rpc.com/?api-key=$HELIUS_API_KEY
 
