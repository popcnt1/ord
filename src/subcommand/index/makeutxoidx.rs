use super::*;
use crate::index::entry::Entry;
use crate::index::SATPOINT_TO_ADDRESS;
use bitcoin::PublicKey;
use bloomfilter::Bloom;
use redb::{Database, ReadableTable, ReadableTableMetadata};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

#[derive(Debug, Parser)]
pub(crate) struct MakeUTXOIdx {
  #[arg(long, help = "utxo list")]
  input: String,
  #[arg(long, help = "utxo:address map database path")]
  output: String,
  #[arg(long, help = "empty address output path for debug")]
  empty_address_output: String,
}

impl MakeUTXOIdx {
  pub(crate) fn run(self, settings: Settings) -> SubcommandResult {
    let start_time = Instant::now();

    // ord outpoint bloomfilter for reducing outpoint:address mapping size and improving get perf
    let index = Index::open(&settings)?;
    index.update()?;
    let rtx = index.database.begin_read()?;
    let satpoint_table = rtx.open_table(crate::index::SEQUENCE_NUMBER_TO_SATPOINT)?;
    let item_count = satpoint_table.len()?;
    let mut bloom = Bloom::new_for_fp_rate(item_count as usize, 0.001);
    let satpoint_iter = satpoint_table.iter()?;
    for result in satpoint_iter {
      let entry = result?;
      let satpoint = SatPoint::load(*entry.1.value());
      let outpoint = satpoint.outpoint;
      bloom.set(&bcs::to_bytes(&outpoint).unwrap());
    }
    let mut writer = BufWriter::new(fs::File::create(self.empty_address_output)?);
    let mut bloom_filter_positive_count: u64 = 0;
    let mut empty_address_count: u64 = 0;
    let mut mapped_count: u64 = 0;
    let utxo_idx = Database::create(self.output.as_str()).unwrap();
    let write_txn = utxo_idx.begin_write().unwrap();
    let mut utxo_reader =
      BufReader::with_capacity(8 * 1024 * 1024, File::open(self.input).unwrap());
    let mut is_title_line = true;
    {
      let mut table = write_txn.open_table(SATPOINT_TO_ADDRESS).unwrap();
      for line in utxo_reader.by_ref().lines() {
        let line = line.unwrap();
        if is_title_line {
          is_title_line = false;
          if line.starts_with("count") {
            continue;
          }
        }
        let (outpoint, address, script_type, script) = derive_utxo_info(line.as_str());
        if !bloom.check(&bcs::to_bytes(&outpoint).unwrap()) {
          continue;
        }
        bloom_filter_positive_count += 1;
        if address.is_empty() {
          empty_address_count += 1;
          writeln!(
            writer,
            "{},{},{}",
            outpoint.to_string(),
            script_type,
            script
          )
          .expect("unable to write data to file");
          continue;
        }
        table
          .insert(
            bcs::to_bytes(&outpoint).unwrap().as_slice(),
            bcs::to_bytes(&address).unwrap().as_slice(),
          )
          .unwrap();
        mapped_count += 1;
        if mapped_count % (1024 * 1024) == 0 {
          println!(
            "mapped_count: {}, cost: {:?}",
            mapped_count,
            start_time.elapsed()
          );
        }
      }
    }

    write_txn.commit().unwrap();

    println!(
      "ord_count: {}， bloom_filter_positive_count: {}, empty_address_count: {}, mapped_count: {}, cost: {:?}",
      item_count, bloom_filter_positive_count, empty_address_count, mapped_count, start_time.elapsed()
    );

    Ok(None)
  }
}

// line format: count,txid,vout,height,coinbase,amount,script,type,address
fn derive_utxo_info(line: &str) -> (OutPoint, String, String, String) {
  let str_list: Vec<&str> = line.trim().split(',').collect();
  if str_list.len() != 9 {
    panic!("Invalid utxo data: {}", line);
  }
  let txid = str_list[1].to_string();
  let vout = str_list[2].parse::<u32>().expect("Invalid vout format");
  let output = format!("{}:{}", txid, vout);
  let script_type = str_list[7].to_string();
  let script = str_list[6].to_string();
  let src_address = str_list[8].to_string();

  let address: Option<Address> = if src_address.is_empty() {
    let script_buf = ScriptBuf::from_hex(&script).unwrap();
    Address::from_script(&script_buf, Network::Bitcoin).ok()
  } else {
    Address::from_str(src_address.as_str())
      .unwrap()
      .require_network(Network::Bitcoin)
      .ok()
  };

  if !address.is_none() {
    return (
      OutPoint::from_str(&output).unwrap(),
      address.unwrap().to_string(),
      script_type,
      script,
    );
  }

  // script maybe p2pk pubkey/script
  if script_type == "p2pk" {
    let pubkey = match PublicKey::from_str(script.as_str()) {
      Ok(pubkey) => pubkey,
      Err(_) => {
        // is script
        let script_buf = ScriptBuf::from_hex(script.as_str()).unwrap();
        script_buf.p2pk_public_key().unwrap()
      }
    };
    let bitcoin_address = bitcoin::Address::p2pkh(&pubkey, Network::Bitcoin);
    return (
      OutPoint::from_str(&output).unwrap(),
      bitcoin_address.to_string(),
      script_type,
      script,
    );
  }

  // return empty address, when:
  // source address is invalid
  // && cannot derive address from script
  // && script is not p2pk script/pubkey
  (
    OutPoint::from_str(&output).unwrap(),
    "".to_string(),
    script_type,
    script,
  )
}
