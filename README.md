# better-secret-math
This package works with [ethnum](https://github.com/nlordell/ethnum-rs) and [prb-math](https://github.com/paulrberg/prb-math) to implement efficient fixed-point math that works with numbers which are considered to have 18 trailing decimals. The criterion crate is used for benchmarking, and it has shown a performance boost of 2x to 3x in most cases (though this has yet to be tested to see if it translates into lower gas costs).

## Usage
`better-secret-math = { git = "https://github.com/securesecrets/better-secret-math" }`

## Sample Performance Differences
[muldiv vs multiply_ratio](/samples/muldiv.svg)

[mul](/samples/mul.svg)

In the one above, we perform 4 multiplication operations using:
- unchecked U256
- converting from Uint256 -> doing checked U256 -> converting back to Uint256
- converting from Uint256 -> unchecked U256 -> converting back to Uint256
- checked Uint256 (this is the same as unchecked Uint256 since the Mul impl for this type uses checked_mul in it)
