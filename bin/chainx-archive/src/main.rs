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
use std::convert::TryInto;
use std::ops::Deref;

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
    let m: substrate_archive::decoder::metadata::Metadata =
        meta.try_into().expect("failed to convert to metadata");
    let lookup_table: substrate_archive::decoder::StorageMetadataLookupTable = m.clone().into();

    let double_map_key1 = substrate_archive::decoder::filter_double_map_key1_types(m);

    // Ensure all key1 of double map are determined.
    //
    // ["Kind", "TradingPairId", "SessionIndex", "T::AccountId", "Chain"]
    //
    // TODO: build the tables from the metadata

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
