#  這個例子需要吧上傳image ，需要使用devenet 付費，因此需要鏈接devenet
- 測試的腳本
1. 在helius.com  申請一個api key
export HELIUS_API_KEY=<YOUR_API_KEY>
2. 執行這個腳本
./anchor-dev-test.sh

腳本的內容如下：
```bash
 anchor test --provider.wallet `solana config get keypair | awk -F ':' '{print $2}' `  --provider.cluster https://devnet.helius-rpc.com/?api-key=$HELIUS_API_KEY

```

