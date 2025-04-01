# [WAVS](https://docs.wavs.xyz) NFT DEMO

**Template for getting started with developing building dynamic NFTs with WAVS.**

This example demonstrates a simple dynamic NFT contract that can be minted / updated via an AVS.

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

### Install Ollama

This example use an LLM configured for determinism, run locally with Ollama. The model is llama3.1, but other open source models can be used if you change the config in `components/automous-artist/src`.

For more information about AVSs and determinstic AI, see our [blog post on the subject](https://www.layer.xyz/news-and-insights/deterministic-ai).

You can download Ollama here: https://ollama.com/

Get the llama 3.1 model.

```bash
ollama pull llama3.1
```

Note: in a production AVS environment, you would need to ship an AVS that bundles WAVS and Ollama together into a new docker image. More information on support for WAVS sidecars will be forthcoming in a future release.

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

## Deploy Service

Deploy the compiled component with the contracts from the previous steps. Review the [makefile](./Makefile) for more details.

`TRIGGER_EVENT` is the event signature that the trigger contract emits and WAVS watches for. By altering `SERVICE_TRIGGER_ADDR` you can watch events for even contracts others have deployed.

The `SERVICE_SUBMISSION_ADDR` is the contract to which results from the AVS are submitted and implements the `IWavsServiceHandler` interface which is simply `function handleSignedData(bytes calldata data, bytes calldata signature) external`.

Let's set these based on our recently run deployment script, and deploy the component.

```bash
# Get deployed service trigger and submission contract addresses
export SERVICE_TRIGGER_ADDR=`jq -r '.minter' "./.docker/script_deploy.json"`
export SERVICE_SUBMISSION_ADDR=`jq -r '.nft' "./.docker/script_deploy.json"`

# Deploy component
COMPONENT_FILENAME=autonomous_artist.wasm TRIGGER_EVENT="AvsMintTrigger(address,string,uint256,uint8)" SERVICE_TRIGGER_ADDR=$SERVICE_TRIGGER_ADDR SERVICE_SUBMISSION_ADDR=$SERVICE_SUBMISSION_ADDR make deploy-service
```

To see all options for deploying services, run `make wavs-cli -- deploy-service -h` and consider customizing `deploy service` in the `Makefile`.

## Trigger the Service

If you're in a new terminal, make sure you have the environment set up. The script will automatically read the minter address from the deployment JSON file.

```bash
export PROMPT="How do I become a great artist?"
forge script ./script/Trigger.s.sol:Trigger "${PROMPT}" --sig "run(string)" --rpc-url http://localhost:8545 --broadcast
```

## Show the result

Query the latest NFT result using the show script:

```bash
forge script ./script/Show.s.sol:ShowResults --rpc-url http://localhost:8545
```

Or manually query the NFT contract:

```bash
export NFT_ADDR=`jq -r '.nft' "./.docker/script_deploy.json"`
cast call $NFT_ADDR "tokenURI(uint256)(string)" 0 | grep -o 'base64,[^"]*' | cut -d',' -f2 | base64 -d | jq
```
