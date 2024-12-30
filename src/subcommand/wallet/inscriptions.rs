use {super::*, crate::wallet::Wallet};
use crate::sat_point::SatPoint;

#[derive(Serialize, Deserialize)]
pub struct Output {
  pub inscription: InscriptionId,
  pub location: SatPoint,
  pub explorer: String,
}

pub(crate) fn run(options: Options) -> SubcommandResult {
  let index = Index::open(&options)?;
  index.update()?;

  let inscriptions = index.get_inscriptions(None)?;
  let unspent_outputs = index.get_unspent_outputs(Wallet::load(&options)?)?;

  let explorer = match options.chain() {
    Chain::Mainnet => "http://localhost/inscription/",
    Chain::Regtest => "http://localhost/inscription/",
    Chain::Signet => "https://localhost/inscription/",
    Chain::Testnet => "http://localhost/inscription/", // Removed extra quote here
  };

  let mut output = Vec::new();

  for (location, inscription) in inscriptions {
    if unspent_outputs.contains_key(&location.outpoint) {
      output.push(Output {
        location,
        inscription,
        explorer: format!("{}{}", explorer, inscription), // Corrected format string
      });
    }
  }

  Ok(Box::new(output))
}
