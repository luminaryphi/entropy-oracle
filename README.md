Entropy-Oracle is based on the decentralized entropy pool used in [Keymaker](URL 'https://github.com/luminaryphi/keymaker'). The premise is a ratchet system where everyone who takes from the pool must also contribute entropy to the pool, updating the internal entropy seed in the process.

To allow your contract to receive entropy from the pool, you must call the oracles `gather_entropy` handle, providing the hash of your contract and a string of entropy. You must make a write a Handle called `ReceiveEntropy` that accepts a `[u8; 32]` called `entropy` such as 

```
pub enum EntropyHandleMsg {
    ReceiveEntropy { entropy: [u8; 32] },
}
```

This is what the oracle will be calling back to give your contract its entropy output.

