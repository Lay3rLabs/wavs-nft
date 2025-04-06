# [WAVS](https://docs.wavs.xyz) NFT DEMO

**Template for getting started with developing building dynamic NFTs with WAVS.**

This example demonstrates a simple Dynamic NFT + Minter contract that can even communicate with each other cross-chain.

There are two conracts `WavsNft.sol` and `WavsMinter.sol`, as well as two components `autonomous-artist` and `simple-relay`.

The flow is:

1. User pays minter contract which emits an `WavsNftTrigger` event
2. WAVS listens for event and triggers the registered WASI component
3. `autonomous-artist` component runs and outputs an NFT tokenURI
4. WAVS operators sign output with their keys and send results to aggregator
5. Aggregator agregates signatures and puts results on chain
6. `handleSignedData` is called on the `WavsNft.sol` contract, it mints an NFT with the tokenURI and emits an `WavsNftMint` event.
7. WAVS listens for event and triggers the registered WASI component
8. `simple-relay` component runs an outputs the TriggerId that has been completed
9. Operators sign output
10. Aggregator agregates signatures and submits them on chain
11. `handleSignedData` is called on the `WavsMinter.sol` contract, it deletes the Receipt

## System Requirements

<details>
<summary>Core (Docker, Compose, Make, JQ, Node v21+)</summary>

### Docker

- **MacOS**: `brew install --cask docker`
- **Linux**: `sudo apt -y install docker.io`
- **Windows WSL**: [docker desktop wsl](https://docs.docker.com/desktop/wsl/#turn-on-docker-desktop-wsl-2) & `sudo chmod 666 /var/run/docker.sock`
- [Docker Documentation](https://docs.docker.com/get-started/get-docker/)

### Docker Compose

- **MacOS**: Already installed with Docker installer
- **Linux + Windows WSL**: `sudo apt-get install docker-compose-v2`
- [Compose Documentation](https://docs.docker.com/compose/)

### Make

- **MacOS**: `brew install make`
- **Linux + Windows WSL**: `sudo apt -y install make`
- [Make Documentation](https://www.gnu.org/software/make/manual/make.html)

### JQ

- **MacOS**: `brew install jq`
- **Linux + Windows WSL**: `sudo apt -y install jq`
- [JQ Documentation](https://jqlang.org/download/)

### Node.js

- **Required Version**: v21+
- [Installation via NVM](https://github.com/nvm-sh/nvm?tab=readme-ov-file#installing-and-updating)
</details>

<details>

<summary>Rust v1.84+</summary>

### Rust Installation

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

rustup toolchain install stable
rustup target add wasm32-wasip2
```

### Upgrade Rust

```bash
# Remove old targets if present
rustup target remove wasm32-wasi || true
rustup target remove wasm32-wasip1 || true

# Update and add required target
rustup update stable
rustup target add wasm32-wasip2
```

</details>

<details>
<summary>Cargo Components</summary>

### Install Cargo Components

```bash
# Install required cargo components
# https://github.com/bytecodealliance/cargo-component#installation
cargo install cargo-binstall
cargo binstall cargo-component warg-cli wkg --locked --no-confirm --force

# Configure default registry
wkg config --default-registry wa.dev
```

</details>

<details>
<summary>Install Ollama</summary>
### Install Ollama

This example use an LLM configured for determinism, run locally with Ollama. The model is llama3.1, but other open source models can be used if you change the config in `components/automous-artist/src`.

For more information about AVSs and determinstic AI, see our [blog post on the subject](https://www.layer.xyz/news-and-insights/deterministic-ai).

You can download Ollama here: https://ollama.com/

Get the llama 3.1 model.

```bash
ollama pull llama3.1
```

Note: in a production AVS environment, you would need to ship an AVS that bundles WAVS and Ollama together into a new docker image. More information on support for WAVS sidecars will be forthcoming in a future release.

</details>

### Solidity

Install the required packages to build the Solidity contracts. This project supports both [submodules](./.gitmodules) and [npm packages](./package.json).

```bash
# Install packages (npm & forge submodules)
make setup

# Build the contracts
forge build

# Run the solidity tests
forge test
```

### Build WASI components

Now build the WASI rust components into the `compiled` output directory.

> [!WARNING]
> If you get: `error: no registry configured for namespace "wavs"`
>
> run, `wkg config --default-registry wa.dev`

```bash
make wasi-build # or `make build` to include solidity compilation.
```

Note: under the hood this uses `cargo component build --release` for each component in the `components` directory and moves them to the `compiled` directory. See `Makefile` for more details.

### Execute WASI component directly

Test run the component locally to validate the business logic works. Nothing will be saved on-chain, just the output of the component is shown.

```bash
PROMPT="How to become a great artist?" make wasi-exec
```

## WAVS

> [!NOTE]
> If you are running on a Mac with an ARM chip, you will need to do the following:
>
> - Set up Rosetta: `softwareupdate --install-rosetta`
> - Enable Rosetta (Docker Desktop: Settings -> General -> enable "Use Rosetta for x86_64/amd64 emulation on Apple Silicon")
>
> Configure one of the following networking:
>
> - Docker Desktop: Settings -> Resources -> Network -> 'Enable Host Networking'
> - `brew install chipmk/tap/docker-mac-net-connect && sudo brew services start chipmk/tap/docker-mac-net-connect`

### Start Environment

Start an ethereum node (anvil), the WAVS service, and deploy [eigenlayer](https://www.eigenlayer.xyz/) contracts to the local network.

```bash
cp .env.example .env

# Start the backend
#
# This must remain running in your terminal. Use another terminal to run other commands.
# You can stop the services with `ctrl+c`. Some MacOS terminals require pressing it twice.
make start-all
```

In a separate terminal, start Ollama:

```
ollama serve
```

### Deploy Contract

Upload your service's trigger and submission contracts. The trigger contract is where WAVS will watch for events, and the submission contract is where the AVS service operator will submit the result on chain.

```bash
export SERVICE_MANAGER_ADDR=`jq -r '.eigen_service_managers.local | .[-1]' .docker/deployments.json`
forge script ./script/Deploy.s.sol:Deploy ${SERVICE_MANAGER_ADDR} --sig "run(string)" --rpc-url http://localhost:8545 --broadcast
```

> [!TIP]
> You can see the deployed NFT address with `jq -r '.nft' "./.docker/script_deploy.json"`,
> the deployed minter address with `jq -r '.minter' "./.docker/script_deploy.json"`,
> and the deployed submission address with `jq -r '.service_handler' "./.docker/script_deploy.json"`

### Deploy Service

Deploy the compiled component with the contracts from the previous steps. Review the [makefile](./Makefile) for more details.

TRIGGER_EVENT is the event signature that the trigger contract emits and WAVS watches for. By altering SERVICE_TRIGGER_ADDR you can watch events for even contracts others have deployed.

The SERVICE_SUBMISSION_ADDR is the contract to which results from the AVS are submitted and implements the IWavsServiceHandler interface which is simply `function handleSignedData(bytes calldata data, bytes calldata signature) external`.

Let's set these based on our recently run deployment script, and deploy the component.

```bash
# Get deployed service trigger and submission contract addresses
export WAVS_MINTER=`jq -r '.minter' "./.docker/script_deploy.json"`
export WAVS_NFT=`jq -r '.nft' "./.docker/script_deploy.json"`

# Deploy autonmous artist component for the minting flow. Triggered here by the WavsMinter.sol contract
COMPONENT_FILENAME=autonomous_artist.wasm TRIGGER_EVENT="WavsNftTrigger(address,string,uint64,uint8,uint256)" SERVICE_TRIGGER_ADDR=$WAVS_MINTER SERVICE_SUBMISSION_ADDR=$WAVS_NFT make deploy-service

# Deploy simple relayer component, triggered by successful minting from the WavsNft.sol contract
COMPONENT_FILENAME=simple_relay.wasm TRIGGER_EVENT="WavsNftMint(address,uint256,string,uint64)" SERVICE_TRIGGER_ADDR=$WAVS_NFT SERVICE_SUBMISSION_ADDR=$WAVS_MINTER make deploy-service

# Deploy autonmous artist component for the update flow. Triggered here by the WavsNft.sol contract
COMPONENT_FILENAME=autonomous_artist.wasm TRIGGER_EVENT="WavsNftTrigger(address,string,uint64,uint8,uint256)" SERVICE_TRIGGER_ADDR=$WAVS_NFT SERVICE_SUBMISSION_ADDR=$WAVS_NFT make deploy-service
```

To see all options for deploying services, run `make wavs-cli -- deploy-service -h` and consider customizing `deploy service` in the `Makefile`.

### Trigger the Service

```bash
# Run the mint script with the minter address and prompt
forge script ./script/Mint.s.sol:Trigger $WAVS_MINTER "How do I become a great Artist?" \
  --sig "run(address,string)" --rpc-url http://localhost:8545 --broadcast
```

### Show the results

After triggering the service, you can check the status of both the NFT and the mint receipts using the Show script:

```bash
# Run the show script with the NFT and minter addresses
forge script ./script/Show.s.sol:ShowResults $WAVS_NFT $WAVS_MINTER \
  --sig "run(address,address)" --rpc-url http://localhost:8545
```

This will display information about:

- The last minted NFT (including its tokenURI and how to decode it)
- All mint receipts in the system (including whether they're fulfilled)

You can also check and decode the NFT's token URI directly in one line:

```bash
cast call $WAVS_NFT "tokenURI(uint256)(string)" 0 | grep -o 'base64,[^"]*' | cut -d',' -f2 | base64 -d | jq
```

### Update an NFT

You can update an existing NFT using the Update script. This will trigger the autonomous artist component to generate new content based on your prompt.

```bash
# Run the update script with the NFT address, token ID, and new prompt
forge script ./script/Update.s.sol:Update $WAVS_NFT 0 "How do I become a great Engineer?" \
  --sig "run(address,uint256,string)" --rpc-url http://localhost:8545 --broadcast
```

This will:

1. Send the update fee (currently 0.01 ETH) to the contract
2. Trigger the update process
3. Emit the `WavsNftTrigger` event with the tokenId parameter
4. The WAVS service will pick up the event and process the update
5. The NFT's tokenURI will be updated with the new AI-generated content
6. Use the queries above to see updated NFT `tokenURI`
