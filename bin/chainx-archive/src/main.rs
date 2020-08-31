// Copyright 2018-2019 Parity Technologies (UK) Ltd.
// This file is part of substrate-archive.

// substrate-archive is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// substrate-archive is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of // MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with substrate-archive.  If not, see <http://www.gnu.org/licenses/>.

mod archive;
mod cli_opts;
mod config;

use anyhow::Result;
use std::collections::HashMap;
use std::convert::TryInto;
use std::ops::Deref;

use substrate_archive::decoder;

use std::fs::{File, OpenOptions};
use std::io::Write;

pub fn open_file(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
}

#[tokio::main]
pub async fn main() -> Result<()> {
    // Why this does not work?
    // let meta: frame_metadata::RuntimeMetadataPrefixed = chainx_runtime::Runtime::metadata();
    // let m: substrate_archive::decoder::metadata::Metadata =
    // meta.try_into().expect("failed to convert to metadata");

    let meta: sp_core::OpaqueMetadata = chainx_runtime::Runtime::metadata().into();
    let meta: &Vec<u8> = meta.deref();
    let meta: frame_metadata::RuntimeMetadataPrefixed =
        codec::Decode::decode(&mut meta.as_slice()).expect("failed to decode metadata prefixed");
    let metadata: decoder::metadata::Metadata =
        meta.try_into().expect("failed to convert to metadata");
    let lookup_table: decoder::StorageMetadataLookupTable = metadata.clone().into();

    let double_map_key1 = decoder::filter_double_map_key1_types(metadata.clone());

    let double_map_key1_length_table: HashMap<String, usize> = vec![
        // u32
        // u32 hex::encode(1u32.encode()).chars().count()
        ("EraIndex", 8),
        ("SessionIndex", 8),
        ("TradingPairId", 8),
        // Kind = [u8; 16]
        ("Kind", 32),
        // Enum Chain
        ("Chain", 2),
        ("T::AccountId", 64),
    ]
    .into_iter()
    .map(|(k, v)| (k.into(), v))
    .collect();

    println!("double_map_key1:{:?}", double_map_key1);
    for key1 in double_map_key1.iter() {
        println!("key1:{:?}", key1);
        if !double_map_key1_length_table.contains_key(key1) {
            panic!("All key1 length of double map should be determined right now");
        }
    }

    // TODO: Ensure all key1 of double map are determined.

    // TODO: build the tables from the metadata
    let storage_value_types = decoder::filter_storage_value_types(metadata);

    let exe_dir = std::env::current_exe().unwrap();
    let output = exe_dir.join("types.rs");
    // let mut output = open_file(&format!("{}", output.display())).unwrap();
    let mut output = open_file("/home/xlc/data/src/github.com/paritytech/substrate-archive/bin/chainx-archive/src/types.rs").unwrap();
    write!(output, "match try_decode_storage_value! {{\n").unwrap();
    write!(output, "    any_type, encoded =>\n").unwrap();
    for value_ty in storage_value_types.iter() {
        let raw_value_ty = value_ty.clone();
        let value_ty = value_ty.replace("\n", "");
        let value_ty = value_ty.replace("T::AccountId", "AccountId");
        let value_ty = value_ty.replace("T::ValidatorId", "AccountId");
        let value_ty = value_ty.replace("T::Authority", "ImOnlineId");
        let value_ty = value_ty.replace("T::Balance", "Balance");
        let value_ty = value_ty.replace("T::BlockNumber", "BlockNumber");
        let value_ty = value_ty.replace("T::Hash", "Hash");
        let value_ty = value_ty.replace("T::Index", "AccountIndex");
        let value_ty = value_ty.replace("T::Price", "Balance");
        let value_ty = value_ty.replace("T::Moment", "Moment");
        let value_ty = value_ty.replace("T::Event", "Event");
        let value_ty = value_ty.replace("T::Keys", "SessionKeys");
        let value_ty = value_ty.replace("T::AccountData", "AccountData");
        let value_ty = value_ty.replace("BalanceOf<T>", "Balance");
        let value_ty = value_ty.replace("BalanceOf<T, I>", "Balance");
        let line = format!("{:?} => {},", raw_value_ty, value_ty);
        write!(output, "        {:?} => {},\n", raw_value_ty, value_ty).unwrap();
    }
    write!(output, "}}\n").unwrap();
    return Ok(());

    let config = config::Config::new()?;
    substrate_archive::init_logger(config.cli().log_level, log::LevelFilter::Debug);

    let archive = archive::run_archive(config.clone()).await?;
    ctrlc().await?;
    Ok(())
}

async fn ctrlc() -> Result<()> {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to listen on ctrlc");
    println!("\nShutting down ...");
    Ok(())
}
