# Diamond

Implementation of a <https://diamondX.network> node in Rust based on the Substrate framework.

> **NOTE:** In 2018, we split our implementation of "diamond" from its development framework
> "Substrate". See the [Substrate][substrate-repo] repo for git history prior to 2018.

[Superchain-repo]: https://github.com/DiamondNetwork/Superchain.git

This repo contains runtimes for the diamond, gold, and ruby networks. The README provides
information about installing the `diamond` binary and developing on the codebase. For more
specific guides, like how to be a validator, see the
[diamond Wiki](https://wiki.diamondX.network/docs/en/).

## Installation

If you just wish to run a diamond node without compiling it yourself, you may
either run the latest binary from our
[releases](https://github.com/DiamondNetwork) page, or install
diamond from one of our package repositories.

Installation from the Debian or rpm repositories will create a `systemd`
service that can be used to run a diamond node. This is disabled by default,
and can be started by running `systemctl start diamond` on demand (use
`systemctl enable diamond` to make it auto-start after reboot). By default, it
will run as the `diamond` user.  Command-line flags passed to the binary can
be customized by editing `/etc/default/diamond`. This file will not be
overwritten on updating diamond. You may also just run the node directly from
the command-line.

### Debian-based (Debian, Ubuntu)

Currently supports Debian 10 (Buster) and Ubuntu 20.04 (Focal), and
derivatives. Run the following commands as the `root` user.

```
# Import the security@parity.io GPG key
gpg --recv-keys --keyserver hkps://keys.mailvelope.com 9D4B2B6EB8F97156D19669A9FF0812D491B96798
gpg --export 9D4B2B6EB8F97156D19669A9FF0812D491B96798 > /usr/share/keyrings/parity.gpg
# Add the Parity repository and update the package index
echo 'deb [signed-by=/usr/share/keyrings/parity.gpg] https://releases.parity.io/deb release main' > /etc/apt/sources.list.d/parity.list
apt update
# Install the `parity-keyring` package - This will ensure the GPG key
# used by APT remains up-to-date
apt install parity-keyring
# Install diamond
apt install diamond

```

### RPM-based (Fedora, CentOS)

Currently supports Fedora 32 and CentOS 8, and derivatives.

```
# Install dnf-plugins-core (This might already be installed)
dnf install dnf-plugins-core
# Add the repository and enable it
dnf config-manager --add-repo https://releases.parity.io/rpm/diamond.repo
dnf config-manager --set-enabled diamond
# Install diamond (You may have to confirm the import of the GPG key, which
# should have the following fingerprint: 9D4B2B6EB8F97156D19669A9FF0812D491B96798)
dnf install diamond
```

## Building

### Install via Cargo

Make sure you have the support software installed from the **Build from Source** section 
below this section.

If you want to install diamond in your PATH, you can do so with with:

```bash
cargo install --git https://github.com/paritytech/diamond --tag <version> diamond --locked
```

### Build from Source

If you'd like to build from source, first install Rust. You may need to add Cargo's bin directory
to your PATH environment variable. Restarting your computer will do this for you automatically.

```bash
curl https://sh.rustup.rs -sSf | sh
```

If you already have Rust installed, make sure you're using the latest version by running:

```bash
rustup update
```

Once done, finish installing the support software:

```bash
sudo apt install build-essential git clang libclang-dev pkg-config libssl-dev
```

Build the client by cloning this repository and running the following commands from the root
directory of the repo:

```bash
git checkout <latest tagged release>
./scripts/init.sh
cargo build --release
```

Note that compilation is a memory intensive process. We recommend having 4 GiB of physical RAM or swap available (keep in mind that if a build hits swap it tends to be very slow).

#### Build from Source with Docker

You can also build from source using 
[Parity CI docker image](https://github.com/paritytech/scripts/tree/master/dockerfiles/ci-linux):

```bash
git checkout <latest tagged release>
docker run --rm -it -w /shellhere/diamond \
                    -v $(pwd):/shellhere/diamond \
                    paritytech/ci-linux:production cargo build --release
sudo chown -R $(id -u):$(id -g) target/
```

If you want to reproduce other steps of CI process you can use the following 
[guide](https://github.com/paritytech/scripts#gitlab-ci-for-building-docker-images).

## Networks

This repo supports runtimes for diamond, gold, and ruby.

### Connect to Diamond Mainnet

Connect to the global diamond Mainnet network by running:

```bash
./target/release/diamond --chain=diamond
```

You can see your node on [telemetry] (set a custom name with `--name "my custom name"`).

[telemetry]: https://telemetry.Polkadot.io/#list/diamond

### Connect to the "Gold" Gold Network

Connect to the global gold canary network by running:

```bash
./target/release/diamond --chain=gold
```

You can see your node on [telemetry] (set a custom name with `--name "my custom name"`).

[telemetry]: https://telemetry.Polkadot.io/#list/gold

### Connect to the Ruby Testnet

Connect to the global ruby testnet by running:

```bash
./target/release/diamond --chain=ruby
```

You can see your node on [telemetry] (set a custom name with `--name "my custom name"`).

[telemetry]: https://diamond.Polkadot.io/#list/ruby

### Obtaining DOTs

If you want to do anything on diamond, gold, or ruby, then you'll need to get an account and
some DOT, KSM, or WND tokens, respectively. See the
[claims instructions](https://claims.diamondX.network/) for diamond if you have DOTs to claim. For
ruby's WND tokens, see the faucet
[instructions](https://wiki.diamondX.network/docs/en/learn-DOT#getting-westies) on the Wiki.

## Hacking on diamond

If you'd actually like to hack on diamond, you can grab the source code and build it. Ensure you have
Rust and the support software installed. This script will install or update Rust and install the
required dependencies (this may take up to 30 minutes on Mac machines):

```bash
curl https://getsubstrate.io -sSf | bash -s -- --fast
```

Then, grab the diamond source code:

```bash
git clone https://github.com/DiamondNetwork/Superchain.git
cd Superchain-main
```

Then build the code. You will need to build in release mode (`--release`) to start a network. Only
use debug mode for development (faster compile times for development and testing).

```bash
./scripts/init.sh   # Install WebAssembly. Update Rust
cargo build # Builds all native code
```

You can run the tests if you like:

```bash
cargo test --all
```

You can start a development chain with:

```bash
cargo run -- --dev
```

Detailed logs may be shown by running the node with the following environment variables set:

```bash
RUST_LOG=debug RUST_BACKTRACE=1 cargo run???--???--dev
```

### Development

You can run a simple single-node development "network" on your machine by running:

```bash
diamond --dev
```

You can muck around by heading to <https://Polkadot.js.org/apps> and choose "Local Node" from the
Settings menu.

### Local Two-node Testnet

If you want to see the multi-node consensus algorithm in action locally, then you can create a
local testnet. You'll need two terminals open. In one, run:

```bash
diamond --chain=diamond-local --alice -d /tmp/alice
```

And in the other, run:

```bash
diamond --chain=diamond-local --bob -d /tmp/bob --port 30334 --bootnodes '/ip4/127.0.0.1/tcp/30333/p2p/ALICE_BOOTNODE_ID_HERE'
```

Ensure you replace `ALICE_BOOTNODE_ID_HERE` with the node ID from the output of the first terminal.

### Using Docker
[Using Docker](doc/docker.md)

### Shell Completion
[Shell Completion](doc/shell-completion.md)

## Contributing

### Contributing Guidelines

[Contribution Guidelines](CONTRIBUTING.md)

### Contributor Code of Conduct

[Code of Conduct](CODE_OF_CONDUCT.md)

## License

diamond is [GPL 3.0 licensed](LICENSE).
