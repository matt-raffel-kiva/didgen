# didgen

Command line tool to generate did, verkey pairs.  


## Building
Install rust. Built using `1.48.0`. Pull the code.    

At the root of project, `cargo build`.

### Note
You need to have libindy installed or built on your system. See [docs](https://github.com/hyperledger/indy-sdk) for more details

## Command Line arguments
`--wallet` (optional), specify the name of the wallet to use  
`--password` (optional), specific the password for the wallet    
`--seed` (optional), specify the cryptographic seed for generating did and verkey. Must be 32 bytes  

## Output

With no arguments used
```
mattraffel@kiva-mattr:~/src/mine/didgen/target/debug$ ./didgen
did    -> BMfctabCSbwGvKsmWVuo8c
verkey -> 6ePERN47hzucxApTkNteyZPoJq9V7oHR4zBPs1Z1y1RG
```

and with arguments, the output is the same
```
mattraffel@kiva-mattr:~/src/mine/didgen/target/debug$ ./didgen --seed=00000000000000000000000Squidward
did    -> SR2GQxr47phoUVwajdsuyS
verkey -> ErPjftNvwrYVTjjYo7tD21uLxMhQR8udgz1QzR2WQsvj
```

and if you run using cargo, pass command line arguments like this
```
mattraffel@kiva-mattr:~/src/mine/didgen$ cargo run -- --seed=00000000000000000000000Squidward
   Compiling didgen v0.1.0 (/Users/mattraffel/src/mine/didgen)
    Finished dev [unoptimized + debuginfo] target(s) in 1.72s
     Running `target/debug/didgen --seed=00000000000000000000000Squidward`
did    -> SR2GQxr47phoUVwajdsuyS
verkey -> ErPjftNvwrYVTjjYo7tD21uLxMhQR8udgz1QzR2WQsvj
```