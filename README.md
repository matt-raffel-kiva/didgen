# didgen

Command line tool to generate did, verkey pairs.  

Intended to follow the [specification](https://www.w3.org/TR/did-1.0/) but probably needs work.

## Building
Install rust. Built using `1.54.0`. Pull the code.    

At the root of project, `cargo build`.

### Note
You need to have libsodium installed or built on your system. 

## Command Line arguments
`--seed` (optional), specify the cryptographic seed for generating did and verkey. Must be 32 bytes  

## Output

note: verkey is also public key.  Keeping name to match [indy](https://github.com/hyperledger/indy-sdk) naming.  TODO: change to public key.

```

With no arguments used
```
mattraffel@kiva-mattr:~/src/mine/didgen/target/debug$ ./didgen
did          -> SR2GQxr47phoUVwajdsuyS
verkey       -> ErPjftNvwrYVTjjYo7tD21uLxMhQR8udgz1QzR2WQsvj
pk           -> xt19s1sp2UZCGhy9rNyb1FtxdKiDGZZX6hUkC4k8nWJKfZGbRrEG19tDjmCAKCaJLA26BN6gvZxikJFYGaqX95d
```

and with arguments, the output is the same
```
mattraffel@kiva-mattr:~/src/mine/didgen/target/debug$ ./didgen --seed=00000000000000000000000Squidward
did          -> SR2GQxr47phoUVwajdsuyS
verkey       -> ErPjftNvwrYVTjjYo7tD21uLxMhQR8udgz1QzR2WQsvj
pk           -> xt19s1sp2UZCGhy9rNyb1FtxdKiDGZZX6hUkC4k8nWJKfZGbRrEG19tDjmCAKCaJLA26BN6gvZxikJFYGaqX95d
```

and if you run using cargo, pass command line arguments like this
```
mattraffel@kiva-mattr:~/src/mine/didgen$ cargo run -- --seed=00000000000000000000000Squidward
   Compiling didgen v0.1.0 (/Users/mattraffel/src/mine/didgen)
    Finished dev [unoptimized + debuginfo] target(s) in 1.72s
     Running `target/debug/didgen --seed=00000000000000000000000Squidward`
did          -> SR2GQxr47phoUVwajdsuyS
verkey       -> ErPjftNvwrYVTjjYo7tD21uLxMhQR8udgz1QzR2WQsvj
pk           -> xt19s1sp2UZCGhy9rNyb1FtxdKiDGZZX6hUkC4k8nWJKfZGbRrEG19tDjmCAKCaJLA26BN6gvZxikJFYGaqX95d
```
