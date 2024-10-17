
## 🦾 What are we going to do?

Ahoy, pirate! 🏴‍☠️ We’re about to embark on a journey through the seas of **Secret Network**, where your privacy is the true treasure. We'll be setting up your development environment to write, deploy, and interact with **Secret Contracts**—powerful smart contracts that keep your data hidden from prying eyes.

Once we’re done with setup, we’ll deploy our first contract, connect it to a React app, and interact with it like a true blockchain captain. Get ready to sail into the unknown!

---

### 🪟 WINDOWS USERS!

If you’re on Windows, you’ll need to use **Windows Subsystem for Linux 2 (WSL2)** to follow this journey. Check out [this guide](https://docs.microsoft.com/en-us/windows/wsl/install) to install WSL2, and follow [this guide](https://www.codingwithcalvin.net/installing-docker-and-docker-compose-in-wsl2ubuntu-on-windows/) to set up Docker Compose. Also, install [Windows Terminal](https://www.microsoft.com/en-us/p/windows-terminal/9n0dx20hk701) for a smoother ride.

---

## 🚢 Install Docker

First things first, we need **Docker** to run **LocalSecret**, our local version of the Secret Network where you’ll be testing and deploying contracts.

1. Download **Docker Desktop** from [here](https://www.docker.com/products/docker-desktop) and follow the installation instructions.
2. Increase the memory allocation to **2.5 GB** in Docker preferences (this is important to avoid any crashes when running LocalSecret).
3. **For Windows users**, make sure to check “Install required Windows components for WSL 2” during installation.

---

## 🛢 Install NodeJS (LTS Version)

We need **NodeJS** (the latest **LTS version**) for building our React app. Let’s install it using **Node Version Manager (NVM)**:

1. Install NVM:
   ```bash
   curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.1/install.sh | bash
   ```

2. Once NVM is installed, install the latest Node LTS version:
   ```bash
   nvm install --lts
   ```

---

## ⚙️ Install Rust

We’re writing our **Secret Contracts** in Rust! Let’s get it installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Next, add the **WASM target** for compiling contracts to WebAssembly (WASM):

```bash
rustup target add wasm32-unknown-unknown
```

---

## 🛠 Install Git and Make

You’ll need **Git** and **Make** to work on Secret Network. Install them with these commands based on your OS:

- **Linux/WSL**:
   ```bash
   sudo apt-get install git make
   ```

- **MacOS**:
   1. Install **Git** by downloading it [here](https://sourceforge.net/projects/git-osx-installer/). Verify the installation:
      ```bash
      git --version
      ```

   2. Install **Make**:
      ```bash
      brew install make
      ```

---

## 🧰 Install SecretCLI

To interact with **Secret Network** from the command line, we’ll use **SecretCLI**. Follow these steps to get it installed:

- **For Linux**:
   ```bash
   wget https://github.com/scrtlabs/SecretNetwork/releases/latest/download/secretcli-Linux
   chmod +x secretcli-Linux
   sudo mv secretcli-Linux /usr/local/bin/secretcli
   ```

- **For Intel MacOS**:
   ```bash
   wget https://github.com/scrtlabs/SecretNetwork/releases/latest/download/secretcli-macOS
   mv secretcli-macOS secretcli
   chmod 755 secretcli
   sudo mv secretcli /usr/local/bin/secretcli
   ```

- **For ARM MacOS (M1/M2)**:
   ```bash
   wget https://github.com/scrtlabs/SecretNetwork/releases/latest/download/secretcli-macOS-arm64
   mv secretcli-macOS-arm64 secretcli
   chmod 755 secretcli
   sudo mv secretcli /usr/local/bin/secretcli
   ```

---

## 🛡️ Install LocalSecret

**LocalSecret** is your own private Secret Network where you can test and deploy contracts.

Run the following command to start LocalSecret:

```bash
docker run -it -p 9091:9091 -p 26657:26657 -p 1317:1317 -p 5000:5000 --name localsecret ghcr.io/scrtlabs/localsecret:latest
```

🚨 **Note:** LocalSecret currently doesn’t support ARM (M1 MacBooks). You’ll need an Intel-based machine or another solution for this.

---

## 🏦 Set Up Your Wallet and Get Test SCRT from LocalSecret Faucet

Before we set sail, you need a **wallet** to hold your SCRT tokens (gas fees). Here’s how to create one:

1. Create a new wallet:
   ```bash
   secretcli keys add mywallet
   ```

   This will generate a **mnemonic phrase**—write it down and keep it safe!

2. Get your wallet address:
   ```bash
   secretcli keys show mywallet -a
   ```

#### Get SCRT from LocalSecret's Faucet

To interact with LocalSecret, we need some test SCRT. Use the faucet to fill up your treasure chest:

```bash
curl http://localhost:5000/faucet?address=<your_wallet_address>
```

Replace `<your_wallet_address>` with the address you got in the previous step.

---

### 🧪 Compile and Deploy Your Secret Contract

Now that the environment is set, it’s time to compile and deploy your **Secret Contract** to **LocalSecret**! Here’s how we’ll do it, captain. 🏴‍☠️

---

### Step 1: Clone the Counter Contract

Let’s start by cloning a basic **counter contract**:

```bash
cargo generate --git https://github.com/scrtlabs/secret-template.git --name my-counter-contract
```

This will create a folder called `my-counter-contract` with all the necessary files.

---

### Step 2: Configure SecretCLI

We need to ensure **SecretCLI** is set to talk to **LocalSecret** properly. Run these commands to configure it:

```bash
secretcli config node http://localhost:26657
secretcli config chain-id secretdev-1
secretcli config keyring-backend test
secretcli config output json
```

---

### Step 3: Compile the Contract

Now, let's compile the contract to **WASM** so it can be uploaded to the blockchain:

```bash
make build
```

If you’re using a different setup (like Windows), you can use:

```bash
cargo build --release --target wasm32-unknown-unknown
cp ./target/wasm32-unknown-unknown/release/*.wasm ./contract.wasm
```

---

### Step 4: Upload the Contract

It’s time to upload the contract to **LocalSecret**! This is like putting your ship into the water, ready to sail:

```bash
secretcli tx compute store contract.wasm.gz --gas 5000000 --from mywallet --chain-id secretdev-1
```

- **`--from <name>`** refers to the wallet sending the transaction, so replace `<name>` with your wallet name (e.g., `mywallet`).
- **`--gas 5000000`** defines the transaction cost (measured in gas).
- **`--chain-id`** refers to which chain we’re uploading to (in this case, **LocalSecret**).

---

### Step 5: Verify the Contract Upload

Once the contract is uploaded, let’s verify that everything worked:

```bash
secretcli query compute list-code
```

You should see something like this:

```json
[
    {
        "code_id": 1,
        "creator": "secret16u7w28vp68qmldffuc89am4f02045zlfsjht90",
        "code_hash": "2658699cea6112052a342d16fd57ac4411cdf1c05cdac3deceba8de0f6ce026d"
    }
]
```

If you see the **code_id** and **code_hash**, congratulations! 🎉 Your contract has successfully been uploaded to the blockchain.

---

### Step 6: Instantiate the Contract

Now that the contract is uploaded, let’s bring it to life (instantiate it):

```bash
secretcli tx compute instantiate <code_id> '{"count": 0}' --from mywallet --label "counter contract" --gas auto --gas-adjustment 1.2
```

---

And there you have it! 🏴‍☠️ Your first **Secret Contract** is now live and ready to interact with on **LocalSecret**. The adventure continues—explore the possibilities of **Secret Network** as you build, test, and expand your dApp. 🌊



