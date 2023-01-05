
use solana_client::rpc_client::RpcClient;
use hex; 
use core::{fmt, fmt::Display, str::FromStr};
use base58::{FromBase58, ToBase58};
use ed25519_dalek_bip32::{SecretKey, PublicKey, ExtendedSecretKey, DerivationPath};

const URL: &str = "https://api.devnet.solana.com";

use walletd_coins::{CryptoCoin, CryptoWallet};
use walletd_bip39::{Language, Mnemonic, MnemonicType, MnemonicHandler};
use walletd_hd_keys::{BIP32, NetworkType};


#[derive(Default)]
pub enum SolanaFormat {
    #[default]
    Standard,
}

impl SolanaFormat {
    pub fn to_string(&self) -> String {
        match self {
            SolanaFormat::Standard => "Standard".to_string(),
        }
    }
}

pub struct SolanaWallet {
    crypto_type: CryptoCoin,
    address_format: SolanaFormat,
    public_address: String,
    private_key: String,
    public_key: String, 
    keypair: [u8; 64],
    network: NetworkType,
    blockchain_client: Option<RpcClient>,
    seed_hex: Option<String>,
}

impl SolanaWallet {
    pub fn public_address_from_public_key(public_key: &Vec<u8>) -> String {
        public_key.to_base58()
    }

    pub fn keypair_base58(private_key: &[u8; 32], public_key: &[u8; 33]) -> String {
        let mut keypair = [0u8; 64];
        keypair[0..32].copy_from_slice(&private_key.as_slice()[0..32]);
        keypair[32..64].copy_from_slice(&public_key.as_slice()[1..33]);
        keypair.to_base58()
    }   
}

impl Display for SolanaWallet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Solana Wallet")?;
        writeln!(f, " Network: {}", self.network)?;
        writeln!(f, " Private Key: {}", self.private_key)?;
        writeln!(f, " Public Key: {}", self.public_key)?;
        writeln!(f, " Address Format: {}", self.address_format.to_string())?;
        writeln!(f, " Public Address: {}", self. public_address)?;
        Ok(())
    }
}

pub struct BlockchainClient {
    blockchain_client: RpcClient,
}

impl BlockchainClient {
  pub fn new(url: &str) -> Result<Self, String> {
    Ok(Self {
      blockchain_client: RpcClient::new(url),
    })
  }
}
