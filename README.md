# [WAVS](https://docs.wavs.xyz) NFT DEMO

**Template for getting started with developing building dynamic NFTs with WAVS. NOT PRODUCTION READY.**

This example demonstrates a simple Dynamic NFT + Minter contract that can even communicate with each other cross-chain.

There are two contracts [`WavsNft.sol`](./src/contracts/WavsNft.sol) and [`WavsMinter.sol`](./src/contracts/WavsMinter.sol), as well as two components [`autonomous-artist`](./components/autonomous-artist/) and [`simple-relay`](./components/simple-relay/), plus a React frontend application for interacting with the contracts.

TODO:

- [ ] Improve art by doing a second llama query to make a stable diffusion prompt
- [ ] Consider generic WAVS components / libs for IPFS upload, LLM, etc.
- [ ] No haiku, just let Deluze be Deluze
- [ ] Make UI less cringe
- [ ] Maybe comfy UI support?
- [ ] Switch to llama-ccp rather than ollama?

Mint flow:

1. User pays `WavsMinter.sol` contract which emits an `WavsNftTrigger` event, user gets a receipt for their purchase, which after a certain timeout period they can use to get a refund if the AVS fails to run.
2. WAVS listens for event and triggers the registered WASI component `autonomous_artist.wasm`
3. `autonomous-artist` component runs, generates description and image, adds different attributes based on EVM queries, uploads NFT metadata to IPFS, and outputs an NFT tokenURI
4. WAVS operators sign output with their keys and send results to aggregator, aggregator agregates signatures and puts results on chain
5. `handleSignedData` is called on the `WavsNft.sol` contract, it mints an NFT with the tokenURI and emits an `WavsNftMint` event.
6. WAVS listens for event and triggers the registered WASI component `simple_relayer.wasm`
7. `simple-relay` component runs an outputs the TriggerId that has been completed
8. Operators sign output, Aggregator aggregates signatures and submits them on chain
9. `handleSignedData` is called on the `WavsMinter.sol` contract, it deletes the Receipt

Update flow:

1. Owner of an NFT calls `triggerUpdate` on `WavsNft.sol` and emits a `WavsNftTrigger` with the update type.
2. WAVS listens for event and triggers the registered WASI component `autonomous_artist.wasm`
3. `autonomous-artist` component runs, generates new description and image, adds different attributes based on EVM queries, uploads NFT metadata to IPFS, and outputs a new NFT tokenURI
4. WAVS operators sign output with their keys and send results to aggregator, aggregator agregates signatures and puts results on chain
5. `handleSignedData` is called on the `WavsNft.sol` contract, it updates the NFT with the tokenURI.

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
<summary>Install Ollama and Stable Diffusion</summary>
### Install Ollama

This example use an LLM configured for determinism, run locally with Ollama. The model is llama3.1, but other open source models can be used if you change the config in `components/automous-artist/src`.

For more information about AVSs and deterministic AI, see our [blog post on the subject](https://www.layer.xyz/news-and-insights/deterministic-ai).

You can download Ollama here: https://ollama.com/

Get the llama 3.1 model.

```bash
ollama pull llama3.1
```

In a separate terminal run Ollama in the background with:

```bash
ollama serve
```

### Install Stable Diffusion

In a separate terminal, run stable diffusion locally.

```bash
git clone https://github.com/AUTOMATIC1111/stable-diffusion-webui
cd stable-diffusion-webui
./webui.sh --api
```

For testing, you can alternately set `WAVS_ENV_SD_API_URL` and `WAVS_ENV_SD_API_KEY` with a stable diffusion API.

### Notes on Production Deployments

In a production AVS environment, you would need to ship an bundles that bundles WAVS, Ollama, and Stable Diffusion together into a new docker image. More information on support for WAVS sidecars will be forthcoming in a future release. For deterministic output, every AVS operator MUST use the same GPU.

</details>

<details>
<summary>IPFS: Lighthouse API keys</summary>
This example currently uses [Lighthouse](https://lighthouse.storage/) to store NFT metadata.

You can get a free API key by signing up, simply set it in your `.env` file.

```
WAVS_ENV_IPFS_API_URL="https://node.lighthouse.storage/api/v0/add"
WAVS_ENV_LIGHTHOUSE_API_KEY="your-lighthouse-api-key"
```

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
> - Enable Rosetta (Docker Desktop: Settings -> General -> enable "Use Rosetta for x86_46/amd64 emulation on Apple Silicon")
>
> Configure one of the following networking:
>
> - Docker Desktop: Settings -> Resources -> Network -> 'Enable Host Networking'
> - `brew install chipmk/tap/docker-mac-net-connect && sudo brew services start chipmk/tap/docker-mac-net-connect`

### Start Environment

Start an Ethereum node (anvil), the WAVS service, and deploy [EigenLayer](https://www.eigenlayer.xyz/) contracts to the local network.

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

`TRIGGER_EVENT` is the event signature that the trigger contract emits and WAVS watches for. By altering `SERVICE_TRIGGER_ADDR` you can watch events for even contracts others have deployed.

The `SERVICE_SUBMISSION_ADDR` is the contract to which results from the AVS are submitted and implements the `IWavsServiceHandler` interface which is simply `function handleSignedData(bytes calldata data, bytes calldata signature) external`.

Let's set these based on our recently run deployment script, and deploy the component.

```bash
# Get deployed service trigger and submission contract addresses
export WAVS_MINTER=`jq -r '.minter' "./.docker/script_deploy.json"`
export WAVS_NFT=`jq -r '.nft' "./.docker/script_deploy.json"`

# Deploy autonmous artist component for the minting flow. Triggered here by the WavsMinter.sol contract
COMPONENT_FILENAME=autonomous_artist.wasm \
TRIGGER_EVENT="WavsNftTrigger(address,string,uint64,uint8,uint256)" \
SERVICE_TRIGGER_ADDR=$WAVS_MINTER \
SERVICE_SUBMISSION_ADDR=$WAVS_NFT \
SERVICE_CONFIG='{"fuel_limit":100000000,"max_gas":5000000,"host_envs":["WAVS_ENV_IPFS_API_URL","WAVS_ENV_LIGHTHOUSE_API_KEY"],"kv":[["nft_contract","'$WAVS_NFT'"]],"workflow_id":"default","component_id":"default"}' \
make deploy-service

# Deploy simple relayer component, triggered by successful minting from the WavsNft.sol contract
COMPONENT_FILENAME=simple_relay.wasm TRIGGER_EVENT="WavsNftMint(address,uint256,string,uint64)" SERVICE_TRIGGER_ADDR=$WAVS_NFT SERVICE_SUBMISSION_ADDR=$WAVS_MINTER make deploy-service

# Deploy autonmous artist component for the update flow. Triggered here by the WavsNft.sol contract
COMPONENT_FILENAME=autonomous_artist.wasm \
TRIGGER_EVENT="WavsNftTrigger(address,string,uint64,uint8,uint256)" \
SERVICE_TRIGGER_ADDR=$WAVS_NFT \
SERVICE_SUBMISSION_ADDR=$WAVS_NFT \
SERVICE_CONFIG='{"fuel_limit":100000000,"max_gas":5000000,"host_envs":["WAVS_ENV_IPFS_API_URL","WAVS_ENV_LIGHTHOUSE_API_KEY"],"kv":[["nft_contract","'$WAVS_NFT'"]],"workflow_id":"default","component_id":"default"}' \
make deploy-service
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

## Frontend Application

This project includes a React frontend that provides a user-friendly interface for interacting with the WAVS NFT contracts.

### Features

- Connect wallet using RainbowKit
- Mint NFTs by providing text prompts
- View pending mints with loading indicators
- Display minted NFTs in a gallery view
- Toast notifications for success and error messages
- Responsive design using TailwindCSS

### Setup and Running

1. Update the contract addresses in `frontend/src/contexts/MintContext.tsx` with your deployed contract addresses.

2. Install dependencies:

   ```bash
   # At the project root
   yarn
   ```

3. Run the frontend development server:

   ```bash
   yarn dev:frontend
   ```

4. Open your browser at `http://localhost:3000` to access the application.

See the [Frontend README](./frontend/README.md) for more details about the frontend application structure and features.
