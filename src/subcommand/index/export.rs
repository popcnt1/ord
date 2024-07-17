use super::*;

#[derive(Debug, Parser)]
pub(crate) struct Export {
  #[arg(long, help = "Write export to <output>")]
  output: String,
  #[arg(long, help = "old output")]
  old_output: Option<String>,
  #[arg(long, help = "old output changes")]
  changes_path: Option<String>,
  #[arg(long, help = "utxo:address map from utxo list")]
  outpoint_address_map: String,
  #[arg(long, help = "Update all old ord no matter satpoint is changed")]
  update_all: Option<bool>,
  #[arg(long, help = "Export sequence number > <gt_sequence>")]
  gt_sequence: Option<u32>,
  #[arg(long, help = "Export sequence number < <lt_sequence>")]
  lt_sequence: Option<u32>,
}

impl Export {
  pub(crate) fn run(self, settings: Settings) -> SubcommandResult {
    let index = Index::open(&settings)?;

    index.update()?;
    index.export(
      &self.output,
      self.old_output,
      self.changes_path,
      &self.outpoint_address_map,
      self.update_all.unwrap_or(false),
      self.gt_sequence,
      self.lt_sequence,
    )?;

    Ok(None)
  }
}
