use std::sync::{Arc, Mutex};

use blake2::Blake2b512;
use clap::Parser;
use digest::{Digest, DynDigest};
use md5::Md5;
use serde_derive::{Deserialize, Serialize};
use sha1::Sha1;
use sha2::{Sha224, Sha256, Sha384, Sha512};

/**
 * configuration.rs
 *
 * Configuration of the program.
 */

#[derive(Serialize, Deserialize, Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct HasherConfig {
    /// The path to be hashed
    #[arg(short, long, default_value_t = String::from("."))]
    pub input_path: String,

    /// The path to output hashes, {sha256}.json
    #[arg(short, long, default_value_t = String::from("./hashes/"))]
    pub output_path: String,

    /// The location of the config file
    #[arg(short, long, default_value_t = String::from("config.toml"))]
    pub config_file: String,

    /// Maximum number of subdirectories to descend when recursing directories
    #[arg(long, default_value_t = 16)]
    pub max_depth: usize,

    /// Whether to calculate a CRC32 hash [default: true]
    #[arg(long, default_value_t = true)]
    pub crc32: bool,

    /// MD5 hash [default: true]
    #[arg(long, default_value_t = true)]
    pub md5: bool,

    /// SHA-1 [default: true]
    #[arg(long, default_value_t = true)]
    pub sha1: bool,

    /// SHA-224 [default: false]
    #[arg(long, default_value_t = false)]
    pub sha224: bool,

    /// SHA-256 [default: false]
    #[arg(long, default_value_t = false)]
    pub sha256: bool,

    /// SHA-384 [default: false]
    #[arg(long, default_value_t = false)]
    pub sha384: bool,

    /// SHA-512 [default: false]
    #[arg(long, default_value_t = false)]
    pub sha512: bool,

    /// Blake2b512 [default: false]
    #[arg(long, default_value_t = false)]
    pub blake2b512: bool,

    /// Whether or not to follow symlinks [default: true]
    #[arg(long, default_value_t = true)]
    pub follow_symlinks: bool,
}

pub fn get_config() -> HasherConfig {
    let args = HasherConfig::parse();

    /*
    if let Ok(cfg_file) = HasherConfig::from_config_file("myconfig.toml") {
        // Success, merge config file with args

    } else {
        warn!("Error reading config file, only using arguments.");
    }
    */

    args
}

pub fn get_hashes<'a>(
    config: &HasherConfig,
) -> Arc<Mutex<Vec<(&'a str, Arc<Mutex<dyn DynDigest + Send>>)>>> {
    let hashes: Arc<Mutex<Vec<(&str, Arc<Mutex<dyn DynDigest + Send>>)>>> =
        Arc::new(Mutex::new(Vec::new()));

    if config.md5 {
        hashes
            .lock()
            .unwrap()
            .push(("md5", Arc::new(Mutex::new(Md5::new()))));
    }

    if config.sha1 {
        hashes
            .lock()
            .unwrap()
            .push(("sha1", Arc::new(Mutex::new(Sha1::new()))));
    }

    if config.sha224 {
        hashes
            .lock()
            .unwrap()
            .push(("sha224", Arc::new(Mutex::new(Sha224::new()))));
    }

    if config.sha256 {
        hashes
            .lock()
            .unwrap()
            .push(("sha256", Arc::new(Mutex::new(Sha256::new()))));
    }

    if config.sha384 {
        hashes
            .lock()
            .unwrap()
            .push(("sha384", Arc::new(Mutex::new(Sha384::new()))));
    }

    if config.sha512 {
        hashes
            .lock()
            .unwrap()
            .push(("sha512", Arc::new(Mutex::new(Sha512::new()))));
    }

    if config.blake2b512 {
        hashes
            .lock()
            .unwrap()
            .push(("blake2b512", Arc::new(Mutex::new(Blake2b512::new()))));
    }

    hashes
}
