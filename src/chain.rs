use bitcoin::util::address::Address;
use bitcoin::{Block, Network, Script};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};
use std::path::{Path, PathBuf};

#[derive(Default, ValueEnum, Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum Chain {
    #[default]
    #[clap(alias("main"))]
    Mainnet,
    #[clap(alias("test"))]
    Testnet,
    Signet,
    Regtest,
}

impl Chain {
    pub(crate) fn network(self) -> Network {
        match self {
            Self::Mainnet => Network::Bitcoin,
            Self::Testnet => Network::Testnet,
            Self::Signet => Network::Signet,
            Self::Regtest => Network::Regtest,
        }
    }

    pub(crate) fn default_rpc_port(self) -> u16 {
        match self {
            Self::Mainnet => 33864,
            Self::Testnet => 44863,
            Self::Signet => 38332,
            Self::Regtest => 18332,
        }
    }

    pub(crate) fn inscription_content_size_limit(self) -> Option<usize> {
        match self {
            Self::Mainnet | Self::Regtest => None,
            Self::Testnet | Self::Signet => None,
        }
    }

    pub(crate) fn first_inscription_height(self) -> u32 {
        match self {
            Self::Mainnet => 0,
            Self::Testnet => 0,
            Self::Signet => 0,
            Self::Regtest => 0,
        }
    }

    pub(crate) fn first_dune_height(self) -> u32 {
        match self {
            Self::Mainnet => 0,
            Self::Testnet => 0,
            Self::Signet => 0,
            Self::Regtest => 0,
        }
    }

    pub(crate) fn genesis_block(self) -> Block {
        let genesis_hex: &str = match self {
            Self::Mainnet => "010000000000000000000000000000000000000000000000000000000000000000000000450f8ab5cade1c53c2ef36d655076fd0a114378cc54678073b966dff820459e40c845f67f0ff0f1ef98901000101000000010000000000000000000000000000000000000000000000000000000000000000ffffffff3704ffff001d01042f4e617364617120746f20416464204d6963726f537472617465677920746f20746865204e617331303020496e646578ffffffff010058850c0200000043410404721c4861d9047841a25fb469f98ba09e8c9134a4e0ebfb5c84f5e6969e35911d39e492dc2dda8dc84983059672cb11794bfd08d591b67b2e15cf6b074b32f0ac00000000",
            Self::Testnet => "010000000000000000000000000000000000000000000000000000000000000000000000450f8ab5cade1c53c2ef36d655076fd0a114378cc54678073b966dff820459e40c845f67f0ff0f1ef98901000101000000010000000000000000000000000000000000000000000000000000000000000000ffffffff3704ffff001d01042f4e617364617120746f20416464204d6963726f537472617465677920746f20746865204e617331303020496e646578ffffffff010058850c0200000043410404721c4861d9047841a25fb469f98ba09e8c9134a4e0ebfb5c84f5e6969e35911d39e492dc2dda8dc84983059672cb11794bfd08d591b67b2e15cf6b074b32f0ac00000000",
            Self::Signet => "010000000000000000000000000000000000000000000000000000000000000000000000450f8ab5cade1c53c2ef36d655076fd0a114378cc54678073b966dff820459e40c845f67f0ff0f1ef98901000101000000010000000000000000000000000000000000000000000000000000000000000000ffffffff3704ffff001d01042f4e617364617120746f20416464204d6963726f537472617465677920746f20746865204e617331303020496e646578ffffffff010058850c0200000043410404721c4861d9047841a25fb469f98ba09e8c9134a4e0ebfb5c84f5e6969e35911d39e492dc2dda8dc84983059672cb11794bfd08d591b67b2e15cf6b074b32f0ac00000000",
            Self::Regtest => "010000000000000000000000000000000000000000000000000000000000000000000000450f8ab5cade1c53c2ef36d655076fd0a114378cc54678073b966dff820459e40c845f67f0ff0f1ef98901000101000000010000000000000000000000000000000000000000000000000000000000000000ffffffff3704ffff001d01042f4e617364617120746f20416464204d6963726f537472617465677920746f20746865204e617331303020496e646578ffffffff010058850c0200000043410404721c4861d9047841a25fb469f98ba09e8c9134a4e0ebfb5c84f5e6969e35911d39e492dc2dda8dc84983059672cb11794bfd08d591b67b2e15cf6b074b32f0ac00000000",
        };

        let genesis_buf = hex::decode(genesis_hex).expect("Invalid hex string");
        bitcoin::consensus::deserialize(&genesis_buf).expect("Failed to deserialize block")
    }

    pub(crate) fn address_from_script(
        self,
        script: &Script,
    ) -> Result<Address, bitcoin::util::address::Error> {
        Address::from_script(script, self.network())
    }

    pub(crate) fn join_with_data_dir(self, data_dir: &Path) -> PathBuf {
        match self {
            Self::Mainnet => data_dir.to_owned(),
            Self::Testnet => data_dir.join("testnet3"),
            Self::Signet => data_dir.join("signet"),
            Self::Regtest => data_dir.join("regtest"),
        }
    }
}

impl Display for Chain {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Mainnet => "mainnet",
                Self::Testnet => "testnet",
                Self::Signet => "signet",
                Self::Regtest => "regtest",
            }
        )
    }
}
