UNDER DEVELOPEMENT Still needs work. 


To set up and run the `wonky-ord-shibainucoin` and `shibacoin` repositories, follow these detailed instructions:

---

### Step 1: Update System and Install Prerequisites
Update your system and install the required packages:

```bash
sudo apt update && sudo apt upgrade -y
sudo apt install build-essential git curl wget libssl-dev pkg-config libclang-dev cmake -y
```

---

### Step 2: Install Rust Programming Language
Ensure you have the latest version of Rust installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup update
rustup default stable
```

---

### Step 3: Clone the Repositories

#### Clone `wonky-ord-shibainucoin`:
```bash
git clone https://github.com/dpaydrc20/wonky-ord-shibainucoin.git
cd wonky-ord-shibainucoin
```

#### Clone `shibacoin`:
```bash
cd ..
git clone https://github.com/shibacoinppc/shibacoin.git
```

---

### Step 4: Build `wonky-ord-shibainucoin`

Go to the `wonky-ord-shibainucoin` directory:

```bash
cd wonky-ord-shibainucoin
cargo build --release
```

---

### Step 5: Install `shibacoin` Core

#### Step 5.1: Install Dependencies for Shibacoin
Make sure you have the dependencies:

```bash
sudo apt install automake libtool bsdmainutils libevent-dev libboost-system-dev libboost-filesystem-dev libboost-chrono-dev libboost-test-dev libboost-thread-dev libminiupnpc-dev libzmq3-dev -y
```

#### Step 5.2: Build and Install `shibacoin`
Go to the `shibacoin` directory and build:

```bash
cd ../shibacoin
./autogen.sh
./configure --with-incompatible-bdb
make -j$(nproc)
sudo make install
```

---

### Step 6: Configure `shibacoin` Node
Set up the configuration file for `shibacoin`:

```bash
mkdir -p ~/.shibacoin
nano ~/.shibacoin/shibacoin.conf
```

Add the following lines to the configuration file:

```plaintext
rpcuser=your_rpc_username
rpcpassword=your_rpc_password
server=1
daemon=1
txindex=1
rpcbind=127.0.0.1
rpcallowip=127.0.0.1
port=33864
```

Save and exit.

Start the `shibacoin` node:

```bash
shibacoind -daemon
```

Check synchronization status:

```bash
shibacoin-cli getblockchaininfo
```

---

### Step 7: Run the Ordinal Indexer
Return to the `wonky-ord-shibainucoin` directory:

```bash
cd ../wonky-ord-shibainucoin
```

Run the Ordinal indexer with your settings:

```bash
./target/release/ord \
    --first-inscription-height=0 \
    --rpc-url=http://your_rpc_username:your_rpc_password@127.0.0.1:33864 \
    --data-dir=/mnt/ord-node/indexer-data-new \
    --index-transactions \
    server
```

---

### Step 8: Verify the Setup
1. Ensure your `shibacoin` node is fully synced using `shibacoin-cli getblockchaininfo`.
2. Access the Ordinal Indexer's web server by navigating to the provided address in the terminal output (e.g., `http://127.0.0.1:PORT`).

---

### Notes:
- Replace `your_rpc_username` and `your_rpc_password` with secure credentials.
- Ensure `/mnt/ord-node/indexer-data-new` exists or use a directory you can access.
- Make sure ports are open if running on a remote server.

