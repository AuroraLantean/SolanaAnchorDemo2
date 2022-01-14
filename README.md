# Anchor Demo2

examples from Anchor https://github.com/project-serum/anchor/tree/master/tests

---
## Install Node
install via NVM or download at https://nodejs.org/en/download/

Confirm the installed NodeJs version is 16.13.0

`$ node -v`

---
## Install Rust

https://www.rust-lang.org/tools/install

If you already have Rust: `$ rustup update stable`

Or install Rust : `$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

Confirm the installed Rust version 1.57.0

`$ rustc --version`

---
## Install Solana

https://docs.solana.com/cli/install-solana-cli-tools

To update existing installed Solana to a newer version:

`solana-install update`

To install Solana the first time:

`$ sh -c "$(curl -sSfL https://release.solana.com/v1.9.4/install)"`

The following output indicates a successful update:
```
downloading v1.9.4 installer
Configuration: /home/solana/.config/solana/install/config.yml
Active release directory: /home/solana/.local/share/solana/install/active_release
* Release version: v1.9.4
* Release URL: https://github.com/solana-labs/solana/releases/download/v1.9.4/solana-release-x86_64-unknown-linux-gnu.tar.bz2
Update successful
```

Confirm the installed Solana version 1.9.4:

`$ solana --version`

---
## Test running Solana Test Validator
`$ solana-test-validator`
then you should see something like...
```
Ledger location: test-ledger
Log: test-ledger/validator.log
⠁ Initializing...
Identity: BPS41o6phpkffnbfdnfnbfdbfnrfnrnrnrnnr
Genesis Hash: EEuUfC8ukiluiiiuikuiyukuy
Version: 1.9.4
Shred Version: 15551
Gossip Address: 127.0.0.1:1024
TPU Address: 127.0.0.1:1027
JSON RPC URL: http://127.0.0.1:8899
⠁ 00:00:20 | Processed Slot: 36 | Confirmed Slot: 36 | Finalized Slot: 4 | Full Snapshot Slot: - | Incremental Snaps
```

## Shut Down Local Network
press control + c

`Note. YOU MUST SHUT DOWN any Solana local network for Anchor to run its own Solana local network as part of testing!`

---
## Install Yarn
Yarn is recommended by Anchor for JavaScript package management. Anchor uses Yarn as its default package management tool.

`$ npm install -g yarn`


---
## Install Anchor

Anchor is a framework for Solana's Sealevel runtime providing several convenient developer tools.
[source](https://project-serum.github.io/anchor/getting-started/installation.html)

`$ yarn global add @project-serum/anchor-cli@0.20.1`

Confirm the installed Anchor version is 0.20.1

`$ anchor --version`

Or build Anchor from source:

`cargo install --git https://github.com/project-serum/anchor --tag v0.20.1 anchor-cli --locked`

On Linux systems you may need to install additional dependencies if cargo install fails. On Ubuntu,

`$ sudo apt-get update && sudo apt-get upgrade && sudo apt-get install -y pkg-config build-essential libudev-dev`

---
## Configure Solana CLI to Localhost
`$ solana config get`

output: 
```
Config File: /home/username/.config/solana/cli/config.yml
RPC URL: http://localhost:8899 
WebSocket URL: ws://localhost:8900/ (computed)
Keypair Path: /home/username/.config/solana/id.json 
Commitment: confirmed 
```

If your configuration is on any different network than the one above (such as devnet or mainnet-beta) you can switch it over to localhost with the following command

`$ solana config set --url localhost`

---
## Setup Local Wallet
```
$ solana-keygen new -o ~/.config/solana/id.json
```
## Confirm Your New Local Wallet
```
$ solana-keygen pubkey ~/.config/solana/id.json
```

## Update your Solana local wallet path in Anchor.toml

```
[provider]
cluster = "localnet"
wallet = "~/.config/solana/id.json"

[programs.localnet]
abc = "<program_id>"

[workspace]
members = ["programs/abc"]

[registry]
url = "https://anchor.projectserum.com"

[scripts]
test = "yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/abc.ts"

```

The default settings above in Anchor.toml assumes your filesystem wallet is located at ~/.config/solana/id.json. If you are using Windows OS, you might need to change it inside your Anchor.toml file to 
```
wallet = "/Users/YOUR_USER_NAME/.config/solana/id.json"
```


---
## Compile and build the Solana program
`$ yarn build`


output:

```
yarn run v1.22.17
$ anchor build
BPF SDK: /home/user1234/.local/share/solana/install/releases/1.9.4/solana-release/bin/sdk/bpf
cargo-build-bpf child: rustup toolchain list -v
cargo-build-bpf child: cargo +bpf build --target bpfel-unknown-unknown --release
    Finished release [optimized] target(s) in 0.17s
cargo-build-bpf child: /home/user1234/.local/share/solana/install/releases/1.9.4/solana-release/bin/sdk/bpf/dependencies/bpf-tools/llvm/bin/llvm-readelf --dyn-symbols /mnt/sda4/0Programming/1Solana/1anchorDemo2/target/deploy/abc.so

To deploy this program:
  $ solana program deploy /mnt/sda4/0Programming/1Solana/1anchorDemo2/target/deploy/abc.so
The program address will default to this keypair (override with --program-id):
  /mnt/sda4/0Programming/1Solana/1anchorDemo2/target/deploy/abc-keypair.json
Done in 0.73s.

```

---
## To Run Test
`$ yarn test`

output:
```
  zero-copy
    ✔ Is creates a zero copy account (411ms)
    ✔ Updates a zero copy account field (411ms)

  2 passing (826ms)

```

See the test file at ./tests/abc.ts

if you uncomment any of those lines below and run the test again
```
  //log2('here log2');
  //console.log('bn(3):', bn(3));
```

output:
```
TypeError: Unknown file extension ".ts" for /mnt/sda4/0Programming/1Solana/1anchorDemo2/tests/abc.ts
    at new NodeError (node:internal/errors:371:5)
    at Object.file: (node:internal/modules/esm/get_format:72:15)
    at defaultGetFormat (node:internal/modules/esm/get_format:85:38)
    at defaultLoad (node:internal/modules/esm/load:13:42)
    at ESMLoader.load (node:internal/modules/esm/loader:303:26)
    at ESMLoader.moduleProvider (node:internal/modules/esm/loader:230:58)
    at new ModuleJob (node:internal/modules/esm/module_job:63:26)
    at ESMLoader.getModuleJob (node:internal/modules/esm/loader:244:11)
    at async Promise.all (index 0)
    at ESMLoader.import (node:internal/modules/esm/loader:281:24)
    at importModuleDynamicallyWrapper (node:internal/vm/module:437:15)
    at formattedImport (/mnt/sda4/0Programming/1Solana/1anchorDemo2/node_modules/mocha/lib/nodejs/esm-utils.js:7:14)
    at Object.exports.requireOrImport (/mnt/sda4/0Programming/1Solana/1anchorDemo2/node_modules/mocha/lib/nodejs/esm-utils.js:48:32)
    at Object.exports.loadFilesAsync (/mnt/sda4/0Programming/1Solana/1anchorDemo2/node_modules/mocha/lib/nodejs/esm-utils.js:88:20)
    at singleRun (/mnt/sda4/0Programming/1Solana/1anchorDemo2/node_modules/mocha/lib/cli/run-helpers.js:125:3)
    at Object.exports.handler (/mnt/sda4/0Programming/1Solana/1anchorDemo2/node_modules/mocha/lib/cli/run.js:374:5)
error Command failed with exit code 1.
info Visit https://yarnpkg.com/en/docs/cli/run for documentation about this command.
error Command failed with exit code 1.
info Visit https://yarnpkg.com/en/docs/cli/run for documentation about this command.

```