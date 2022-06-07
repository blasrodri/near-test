## Benchmark results

### verify_ed25519

```
near call sitoula-test.testnet verify_ed25519 '{"signature_p1": [145,193,203,18,114,227,14,117,33,213,121,66,130,14,25,4,36,120,46,142,226,215,7,66,122,112,97,30,249,135,61,165], "signature_p2": [221,249,252,23,105,40,56,70,31,152,236,141,154,122,207,20,75,118,79,90,168,6,221,122,213,29,126,196,216,104,191,6], "msg": [107,97,106,100,108,102,107,106,97,108,107,102,106,97,107,108,102,106,100,107,108,97,100,106,102,107,108,106,97,100,115,107], "iterations": 10}' --accountId sitoula-test.testnet --gas 300000000000000

# transaction id DZMuFHisupKW42w3giWxTRw5nhBviPu4YZLgKZ6cK4Uq
```

### verify_schnorrkel

```
near call sitoula-test.testnet verify_schnorrkel '{"signature_p1": [106, 144, 17, 34, 142, 65, 191, 241, 233, 250, 132, 168, 204, 173, 122, 196, 118, 248, 159, 159, 254, 37, 153, 84, 248, 104, 206, 217, 168, 65, 12, 74], "signature_p2": [183, 134, 143, 30, 123, 61, 112, 153, 244, 109, 199, 195, 164, 0, 7, 55, 26, 199, 164, 219, 147, 217, 157, 239, 198, 108, 162, 246, 52, 49, 116, 132], "msg": [107,97,106,100,108,102,107,106,97,108,107,102,106,97,107,108,102,106,100,107,108,97,100,106,102,107,108,106,97,100,115,107], "iterations": 10}' --accountId sitoula-test.testnet --gas 300000000000000

# transaction id DpWpK2jnyBRgZ7fScaBKULikJyvmMjDHx7AyWMrJL5VB
```

### verify_ecdsa

```
near call sitoula-test.testnet verify_ecdsa '{"signature_p1": [231, 117, 17, 89, 49, 142, 111, 201, 161, 107, 167, 147, 215, 167, 196, 226, 200, 176, 184, 62, 196, 240, 210, 137, 77, 198, 90, 97, 201, 212, 96, 229], "signature_p2": [1, 31, 7, 121, 178, 247, 150, 131, 108, 250, 173, 71, 100, 192, 83, 64, 145, 85, 254, 69, 176, 7, 114, 89, 64, 205, 30, 243, 193, 78, 142, 27], "msg": [107,97,106,100,108,102,107,106,97,108,107,102,106,97,107,108,102,106,100,107,108,97,100,106,102,107,108,106,97,100,115,107], "iterations": 10}' --accountId sitoula-test.testnet --gas 300000000000000
# transaction id AKFwCuQ8g2a8t9SX7vcve348BvwoU3M42MSef7gy1kPA
```

With `iterations = 130` all these calls return `ExecutionError: 'Exceeded the maximum amount of gas allowed to burn per contract.'`
With `iterations = 50` these are the results:

```
ed25518: tx id 6DcJYfkp9fGxDGtQLZ2m6PEDBwKHXpk7Lf5VgDYLi9vB (299 Tgas)
schnorrkel: tx id ACYJh7YC4pQM8fu7DsjmmkSb3WSvikfQkVyej3htiuDQ (304 Tgas)
ecdsa: tx id 2FhtFEbakuwQyHWcyeRoPkCDgXz4erTYSD75QkxDTb2e (185 Tgas)
```
