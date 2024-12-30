use bitcoin::util::address;
use {super::*, clap::ValueEnum};

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
      Self::Mainnet => 33863,
      Self::Regtest => 18332,
      Self::Signet => 38332,
      Self::Testnet => 44863,
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
      Self::Regtest => 0,
      Self::Signet => 0,
      Self::Testnet => 0,
    }
  }

  pub(crate) fn first_dune_height(self) -> u32 {
    match self {
      Self::Mainnet => 0,
      Self::Regtest => 0,
      Self::Signet => 0,
      Self::Testnet => 0,
    }
  }

  pub(crate) fn genesis_block(self) -> Block {
    let genesis_hex: &str = match self {
      Self::Mainnet => "010000000000000000000000000000000000000000000000000000000000000000000000e2cb4861002feac94c0c06971f12aa8b2c82fb3e93244690d5cb399aa51b2ad2ac7f11800900000000ffffffff5104ffff001d01044853686962612047656e657369732020626c6f636b0000000000ffffffff0100f2052a01000000434104678afdb0fe5548271967f1a67130b7105cd6a828e03909a67962e0ea1f61deb649f6bc3f4cef38c4f35504e51ec112de5c384df7ba0b8d578a4c702b6bf11d5fac00000000",
      Self::Regtest => "010000000000000000000000000000000000000000000000000000000000000000000000e2cb4861002feac94c0c06971f12aa8b2c82fb3e93244690d5cb399aa51b2ad2ac7f11800900000000ffffffff5104ffff001d01044853686962612047656e657369732020626c6f636b0000000000ffffffff0100f2052a01000000434104678afdb0fe5548271967f1a67130b7105cd6a828e03909a67962e0ea1f61deb649f6bc3f4cef38c4f35504e51ec112de5c384df7ba0b8d578a4c702b6bf11d5fac00000000",
      Self::Signet => "010000000000000000000000000000000000000000000000000000000000000000000000696ad20e2dd4365c7459b4a4a5af743d5e92c6da3229e6532cd605f6533f2a5bb9a7f052f0ff0f1ef7390f000101000000010000000000000000000000000000000000000000000000000000000000000000ffffffff1004ffff001d0104084e696e746f6e646fffffffff010058850c020000004341040184710fa689ad5023690c80f3a49c8f13f8d45b8c857fbcbc8bc4a8e4d3eb4b10f4d4604fa08dce601aaf0f470216fe1b51850b4acf21b179c45070ac7b03a9ac00000000",
      Self::Testnet => "010000000000000000000000000000000000000000000000000000000000000000000000e2cb4861002feac94c0c06971f12aa8b2c82fb3e93244690d5cb399aa51b2ad2ac7f11800900000000ffffffff5104ffff001d01044853686962612047656e657369732020626c6f636b0000000000ffffffff0100f2052a01000000434104678afdb0fe5548271967f1a67130b7105cd6a828e03909a67962e0ea1f61deb649f6bc3f4cef38c4f35504e51ec112de5c384df7ba0b8d578a4c702b6bf11d5fac00000000",
    };
    let genesis_buf: Vec<u8> = hex::decode(genesis_hex).unwrap();
    bitcoin::consensus::deserialize(&genesis_buf).unwrap()
  }

  pub(crate) fn address_from_script(
    self,
    script: &Script,
  ) -> Result<Address, address::Error> {
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
        Self::Regtest => "regtest",
        Self::Signet => "signet",
        Self::Testnet => "testnet",
      }
    )
  }
}
