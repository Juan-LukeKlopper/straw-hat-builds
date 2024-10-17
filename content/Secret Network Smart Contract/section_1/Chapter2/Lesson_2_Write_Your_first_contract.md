## 🚨 Disclaimer  
**From this point forward, the project is only a demo** showcasing how to use the platform. It doesn’t represent a complete project, as I ran out of time for the full lesson series.

---

By now, you've experienced just how powerful **SecretCLI** can be, handling everything from compiling to deployment. Behind the scenes, it takes care of these steps:

1. **Builds** the contract.
2. **Uploads** the contract to **LocalSecret**.
3. **Instantiates** the contract.

Let’s dive a little deeper into each of these steps.

### 1. **Building the Contract**
Our contracts are written in Rust, but blockchains like **Secret Network** use **WASM**. Building the contract compiles it into this WASM bytecode, making it executable on the network.

### 2. **Uploading the Contract**
Using **SecretCLI**, we uploaded the contract to **LocalSecret** with the following command:

```bash
secretcli tx compute store contract.wasm.gz --from mywallet --gas 5000000 --chain-id secretdev-1
```

This uploads the contract’s code to the blockchain, where it’s now ready for instantiation.

### 3. **Instantiating the Contract**
To bring the contract to life, we called **instantiate**:

```bash
secretcli tx compute instantiate <code_id> '{"count": 0}' --from mywallet --label "counter contract" --gas auto --gas-adjustment 1.2
```

Instantiation sets the initial state of the contract.

---

## 📝 Writing Your Own Smart Contract

Now that you’ve seen how to deploy a pre-made contract, let’s dive into writing your own. We’ll use the **counter contract** as a template, housed in the `my-counter-contract` directory.

Inside the `src` folder, you’ll find:

- `contract.rs`: The core logic of the contract.
- `msg.rs`: The contract’s interface, defining the messages it accepts.
- `state.rs`: The structure of the data it holds.

Let’s start with a basic contract that increments a counter. In `contract.rs`, we define an **instantiate** function that initializes the contract, setting the counter’s starting value:

```rust
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{InstantiateMsg};
use crate::state::{State, STATE};

const CONTRACT_NAME: &str = "crates.io:clicker";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        count: msg.count,
        owner: info.sender.clone(),
    };

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string()))
}
```

This code initializes the contract and sets up its state. 

---

## 🚀 Interacting with the Contract

Now that the contract is deployed, let’s interact with it. Open up **SecretCLI** and run the following command to query the contract's state:

```bash
secretcli query compute list-code
```

To interact with the contract’s `getCount` function, you would use:

```bash
secretcli query compute query <contract_address> '{"get_count": {}}'
```

If everything was set up correctly, the contract will return the current count!

---

With that, you’ve learned the basics of deploying and interacting with a contract using **SecretCLI** and **LocalSecret**. Now that you’ve mastered the essentials, the real adventure begins as you start writing more complex contracts! Keep exploring the blockchain seas! 🌊

---
