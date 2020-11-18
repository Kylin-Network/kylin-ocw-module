# kylin-ocw-module
kylin ocw repo: the pallets demonstrating
concepts, APIs and structures common to most offchain workers.

Run `cargo doc --package offchain-worker --open` to view this module's
documentation.

- [`pallet_example_offchain_worker::Trait`](https://docs.rs/pallet-offchain-worker/latest/pallet_example_offchain_worker/trait.Trait.html)
- [`Call`](https://docs.rs/pallet-offchain-worker/latest/pallet_example_offchain_worker/enum.Call.html)
- [`Module`](https://docs.rs/pallet-offchain-worker/latest/pallet_example_offchain_worker/struct.Module.html)


## Overview

In this example we are going to build a very simplistic, naive and definitely NOT
production-ready oracle for BTC/USD price.
Offchain Worker (OCW) will be triggered after every block, fetch the current price
and prepare either signed or unsigned transaction to feed the result back on chain.
The on-chain logic will simply aggregate the results and store last `64` values to compute
the average price.
Additional logic in OCW is put in place to prevent spamming the network with both signed
and unsigned transactions, and custom `UnsignedValidator` makes sure that there is only
one unsigned transaction floating in the network.

License: Apache 2.0

### Usage
reference runtime/src/lib.rs, or reference recipes/runtimes/ocw-runtime/src/lib.rs

when use offchain, need specific transaction's keys, they will query from substarte's keystore, so need use insertKey to add certain keytype, like ocw's keyType=ocpf;

### Test account
alice
```bash
$ subkey inspect "bottom drive obey lake curtain smoke basket hold race lonely fit walk" --scheme ed25519
Secret phrase `bottom drive obey lake curtain smoke basket hold race lonely fit walk` is account:
  Secret seed:      0xfac7959dbfe72f052e5a0c3c8d6530f202b02fd8f9f5ca3580ec8deb7797479e
  Public key (hex): 0x345071da55e5dccefaaa440339415ef9f2663338a38f7da0df21be5ab4e055ef
  Account ID:       0x345071da55e5dccefaaa440339415ef9f2663338a38f7da0df21be5ab4e055ef
  SS58 Address:     5DFJF7tY4bpbpcKPJcBTQaKuCDEPCpiz8TRjpmLeTtweqmXL

$ subkey inspect "bottom drive obey lake curtain smoke basket hold race lonely fit walk" --scheme  sr25519
Secret phrase `bottom drive obey lake curtain smoke basket hold race lonely fit walk` is account:
  Secret seed:      0xfac7959dbfe72f052e5a0c3c8d6530f202b02fd8f9f5ca3580ec8deb7797479e
  Public key (hex): 0x46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a
  Account ID:       0x46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a
  SS58 Address:     5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV

```

### OCW transaction's Q&A
```bash
WARN (offchain call) Error submitting a transaction to the pool: Pool(UnknownTransaction(UnknownTransaction::NoUnsignedValidator))
```
UnsigneTransaction，must the validator can submit，you need use validator's key in ocw.

```bash
WARN (offchain call) Error submitting a transaction to the pool: Pool(InvalidTransaction(InvalidTransaction::Payment)) 
```
fee is not enough, please charge money.
