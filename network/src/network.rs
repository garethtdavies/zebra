//! Bitcoin network
//! https://www.anintegratedworld.com/unravelling-the-mysterious-block-chain-magic-number/

use compact::Compact;
use chain::Block;
use primitives::hash::H256;
use primitives::bigint::U256;
use {ConsensusFork};

const MAGIC_MAINNET: u32 = 0xD9B4BEF9;
const MAGIC_TESTNET: u32 = 0x0709110B;
const MAGIC_REGTEST: u32 = 0xDAB5BFFA;
const MAGIC_UNITEST: u32 = 0x00000000;

const BITCOIN_CASH_MAGIC_MAINNET: u32 = 0xE8F3E1E3;
const BITCOIN_CASH_MAGIC_TESTNET: u32 = 0xF4F3E5F4;
const BITCOIN_CASH_MAGIC_REGTEST: u32 = 0xFABFB5DA;

const ZCASH_MAGIC_MAINNET: u32 = 0x6427e924;
const ZCASH_MAGIC_TESTNET: u32 = 0xbff91afa;
const ZCASH_MAGIC_REGTEST: u32 = 0x5f3fe8aa;

lazy_static! {
	static ref MAX_BITS_MAINNET: U256 = "00000000ffffffffffffffffffffffffffffffffffffffffffffffffffffffff".parse()
		.expect("hardcoded value should parse without errors");
	static ref MAX_BITS_TESTNET: U256 = "00000000ffffffffffffffffffffffffffffffffffffffffffffffffffffffff".parse()
		.expect("hardcoded value should parse without errors");
	static ref MAX_BITS_REGTEST: U256 = "7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".parse()
		.expect("hardcoded value should parse without errors");

	static ref ZCASH_MAX_BITS_MAINNET: U256 = "0007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".parse()
		.expect("hardcoded value should parse without errors");
	static ref ZCASH_MAX_BITS_TESTNET: U256 = "07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".parse()
		.expect("hardcoded value should parse without errors");
	static ref ZCASH_MAX_BITS_REGTEST: U256 = "0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f".parse()
		.expect("hardcoded value should parse without errors");
}

/// Network magic type.
pub type Magic = u32;

/// Bitcoin [network](https://bitcoin.org/en/glossary/mainnet)
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Network {
	/// The original and main network for Bitcoin transactions, where satoshis have real economic value.
	Mainnet,
	/// The main bitcoin testnet.
	Testnet,
	/// Bitcoin regtest network.
	Regtest,
	/// Testnet for unittests, proof of work difficulty is almost 0
	Unitest,
	/// Any other network. By default behaves like bitcoin mainnet.
	Other(u32),
}

impl Network {
	pub fn magic(&self, fork: &ConsensusFork) -> Magic {
		match (fork, *self) {
			(&ConsensusFork::BitcoinCash(_), Network::Mainnet) => BITCOIN_CASH_MAGIC_MAINNET,
			(&ConsensusFork::BitcoinCash(_), Network::Testnet) => BITCOIN_CASH_MAGIC_TESTNET,
			(&ConsensusFork::BitcoinCash(_), Network::Regtest) => BITCOIN_CASH_MAGIC_REGTEST,
			(&ConsensusFork::ZCash(_), Network::Mainnet) => ZCASH_MAGIC_MAINNET,
			(&ConsensusFork::ZCash(_), Network::Testnet) => ZCASH_MAGIC_TESTNET,
			(&ConsensusFork::ZCash(_), Network::Regtest) => ZCASH_MAGIC_REGTEST,
			(_, Network::Mainnet) => MAGIC_MAINNET,
			(_, Network::Testnet) => MAGIC_TESTNET,
			(_, Network::Regtest) => MAGIC_REGTEST,
			(_, Network::Unitest) => MAGIC_UNITEST,
			(_, Network::Other(value)) => value,
		}
	}

	pub fn max_bits(&self, fork: &ConsensusFork) -> U256 {
		match (fork, *self) {
			(&ConsensusFork::ZCash(_), Network::Mainnet) => ZCASH_MAX_BITS_MAINNET.clone(),
			(&ConsensusFork::ZCash(_), Network::Testnet) => ZCASH_MAX_BITS_TESTNET.clone(),
			(&ConsensusFork::ZCash(_), Network::Testnet) => ZCASH_MAX_BITS_REGTEST.clone(),
			(_, Network::Mainnet) | (_, Network::Other(_)) => MAX_BITS_MAINNET.clone(),
			(_, Network::Testnet) => MAX_BITS_TESTNET.clone(),
			(_, Network::Regtest) => MAX_BITS_REGTEST.clone(),
			(_, Network::Unitest) => Compact::max_value().into(),
		}
	}

	pub fn port(&self, fork: &ConsensusFork) -> u16 {
		match (fork, *self) {
			(&ConsensusFork::ZCash(_), Network::Mainnet) | (&ConsensusFork::ZCash(_), Network::Other(_)) => 8233,
			(&ConsensusFork::ZCash(_), Network::Testnet) => 18233,
			(&ConsensusFork::ZCash(_), Network::Regtest) | (&ConsensusFork::ZCash(_), Network::Unitest) => 18344,
			(_, Network::Mainnet) | (_, Network::Other(_)) => 8333,
			(_, Network::Testnet) => 18333,
			(_, Network::Regtest) | (_, Network::Unitest) => 18444,
		}
	}

	pub fn rpc_port(&self) -> u16 {
		match *self {
			Network::Mainnet | Network::Other(_) => 8332,
			Network::Testnet => 18332,
			Network::Regtest | Network::Unitest => 18443,
		}
	}

	pub fn genesis_block(&self, fork: &ConsensusFork) -> Block {
		match (fork, *self) {
			// TODO
			(&ConsensusFork::ZCash(_), Network::Mainnet) | (&ConsensusFork::ZCash(_), Network::Other(_)) => {
				use serialization;
				use chain;
				use chain::hex::FromHex;
				let origin = "040000000000000000000000000000000000000000000000000000000000000000000000db4d7a85b768123f1dff1d4c4cece70083b2d27e117b4ac2e31d087988a5eac4000000000000000000000000000000000000000000000000000000000000000090041358ffff071f5712000000000000000000000000000000000000000000000000000000000000fd4005000a889f00854b8665cd555f4656f68179d31ccadc1b1f7fb0952726313b16941da348284d67add4686121d4e3d930160c1348d8191c25f12b267a6a9c131b5031cbf8af1f79c9d513076a216ec87ed045fa966e01214ed83ca02dc1797270a454720d3206ac7d931a0a680c5c5e099057592570ca9bdf6058343958b31901fce1a15a4f38fd347750912e14004c73dfe588b903b6c03166582eeaf30529b14072a7b3079e3a684601b9b3024054201f7440b0ee9eb1a7120ff43f713735494aa27b1f8bab60d7f398bca14f6abb2adbf29b04099121438a7974b078a11635b594e9170f1086140b4173822dd697894483e1c6b4e8b8dcd5cb12ca4903bc61e108871d4d915a9093c18ac9b02b6716ce1013ca2c1174e319c1a570215bc9ab5f7564765f7be20524dc3fdf8aa356fd94d445e05ab165ad8bb4a0db096c097618c81098f91443c719416d39837af6de85015dca0de89462b1d8386758b2cf8a99e00953b308032ae44c35e05eb71842922eb69797f68813b59caf266cb6c213569ae3280505421a7e3a0a37fdf8e2ea354fc5422816655394a9454bac542a9298f176e211020d63dee6852c40de02267e2fc9d5e1ff2ad9309506f02a1a71a0501b16d0d36f70cdfd8de78116c0c506ee0b8ddfdeb561acadf31746b5a9dd32c21930884397fb1682164cb565cc14e089d66635a32618f7eb05fe05082b8a3fae620571660a6b89886eac53dec109d7cbb6930ca698a168f301a950be152da1be2b9e07516995e20baceebecb5579d7cdbc16d09f3a50cb3c7dffe33f26686d4ff3f8946ee6475e98cf7b3cf9062b6966e838f865ff3de5fb064a37a21da7bb8dfd2501a29e184f207caaba364f36f2329a77515dcb710e29ffbf73e2bbd773fab1f9a6b005567affff605c132e4e4dd69f36bd201005458cfbd2c658701eb2a700251cefd886b1e674ae816d3f719bac64be649c172ba27a4fd55947d95d53ba4cbc73de97b8af5ed4840b659370c556e7376457f51e5ebb66018849923db82c1c9a819f173cccdb8f3324b239609a300018d0fb094adf5bd7cbb3834c69e6d0b3798065c525b20f040e965e1a161af78ff7561cd874f5f1b75aa0bc77f720589e1b810f831eac5073e6dd46d00a2793f70f7427f0f798f2f53a67e615e65d356e66fe40609a958a05edb4c175bcc383ea0530e67ddbe479a898943c6e3074c6fcc252d6014de3a3d292b03f0d88d312fe221be7be7e3c59d07fa0f2f4029e364f1f355c5d01fa53770d0cd76d82bf7e60f6903bc1beb772e6fde4a70be51d9c7e03c8d6d8dfb361a234ba47c470fe630820bbd920715621b9fbedb49fcee165ead0875e6c2b1af16f50b5d6140cc981122fcbcf7c5a4e3772b3661b628e08380abc545957e59f634705b1bbde2f0b4e055a5ec5676d859be77e20962b645e051a880fddb0180b4555789e1f9344a436a84dc5579e2553f1e5fb0a599c137be36cabbed0319831fea3fddf94ddc7971e4bcf02cdc93294a9aab3e3b13e3b058235b4f4ec06ba4ceaa49d675b4ba80716f3bc6976b1fbf9c8bf1f3e3a4dc1cd83ef9cf816667fb94f1e923ff63fef072e6a19321e4812f96cb0ffa864da50ad74deb76917a336f31dce03ed5f0303aad5e6a83634f9fcc371096f8288b8f02ddded5ff1bb9d49331e4a84dbe1543164438fde9ad71dab024779dcdde0b6602b5ae0a6265c14b94edd83b37403f4b78fcd2ed555b596402c28ee81d87a909c4e8722b30c71ecdd861b05f61f8b1231795c76adba2fdefa451b283a5d527955b9f3de1b9828e7b2e74123dd47062ddcc09b05e7fa13cb2212a6fdbc65d7e852cec463ec6fd929f5b8483cf3052113b13dac91b69f49d1b7d1aec01c4a68e41ce1570101000000010000000000000000000000000000000000000000000000000000000000000000ffffffff4d04ffff071f0104455a6361736830623963346565663862376363343137656535303031653335303039383462366665613335363833613763616331343161303433633432303634383335643334ffffffff010000000000000000434104678afdb0fe5548271967f1a67130b7105cd6a828e03909a67962e0ea1f61deb649f6bc3f4cef38c4f35504e51ec112de5c384df7ba0b8d578a4c702b6bf11d5fac00000000";
				let origin = origin.from_hex().unwrap();
				let genesis: chain::Block = serialization::deserialize_with_flags(&origin as &[u8], serialization::DESERIALIZE_ZCASH).unwrap();
				genesis
			},
			(&ConsensusFork::ZCash(_), Network::Testnet) =>
				"".into(),
			(&ConsensusFork::ZCash(_), Network::Regtest) | (&ConsensusFork::ZCash(_), Network::Unitest) =>
				"".into(),

			(_, Network::Mainnet) | (_, Network::Other(_)) => "0100000000000000000000000000000000000000000000000000000000000000000000003ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323a9fb8aa4b1e5e4a29ab5f49ffff001d1dac2b7c0101000000010000000000000000000000000000000000000000000000000000000000000000ffffffff4d04ffff001d0104455468652054696d65732030332f4a616e2f32303039204368616e63656c6c6f72206f6e206272696e6b206f66207365636f6e64206261696c6f757420666f722062616e6b73ffffffff0100f2052a01000000434104678afdb0fe5548271967f1a67130b7105cd6a828e03909a67962e0ea1f61deb649f6bc3f4cef38c4f35504e51ec112de5c384df7ba0b8d578a4c702b6bf11d5fac00000000".into(),
			(_, Network::Testnet) => "0100000000000000000000000000000000000000000000000000000000000000000000003ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323a9fb8aa4b1e5e4adae5494dffff001d1aa4ae180101000000010000000000000000000000000000000000000000000000000000000000000000ffffffff4d04ffff001d0104455468652054696d65732030332f4a616e2f32303039204368616e63656c6c6f72206f6e206272696e6b206f66207365636f6e64206261696c6f757420666f722062616e6b73ffffffff0100f2052a01000000434104678afdb0fe5548271967f1a67130b7105cd6a828e03909a67962e0ea1f61deb649f6bc3f4cef38c4f35504e51ec112de5c384df7ba0b8d578a4c702b6bf11d5fac00000000".into(),
			(_, Network::Regtest) | (_, Network::Unitest) => "0100000000000000000000000000000000000000000000000000000000000000000000003ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323a9fb8aa4b1e5e4adae5494dffff7f20020000000101000000010000000000000000000000000000000000000000000000000000000000000000ffffffff4d04ffff001d0104455468652054696d65732030332f4a616e2f32303039204368616e63656c6c6f72206f6e206272696e6b206f66207365636f6e64206261696c6f757420666f722062616e6b73ffffffff0100f2052a01000000434104678afdb0fe5548271967f1a67130b7105cd6a828e03909a67962e0ea1f61deb649f6bc3f4cef38c4f35504e51ec112de5c384df7ba0b8d578a4c702b6bf11d5fac00000000".into(),
		}
	}

	pub fn default_verification_edge(&self, fork: &ConsensusFork) -> H256 {
		match *self {
			Network::Mainnet => H256::from_reversed_str("0000000000000000030abc968e1bd635736e880b946085c93152969b9a81a6e2"),
			Network::Testnet => H256::from_reversed_str("000000000871ee6842d3648317ccc8a435eb8cc3c2429aee94faff9ba26b05a0"),
			_ => self.genesis_block(fork).hash(),
		}
	}
}

#[cfg(test)]
mod tests {
	use compact::Compact;
	use {ConsensusFork};
	use super::{
		Network, MAGIC_MAINNET, MAGIC_TESTNET, MAGIC_REGTEST, MAGIC_UNITEST,
		MAX_BITS_MAINNET, MAX_BITS_TESTNET, MAX_BITS_REGTEST,
	};

	#[test]
	fn test_network_magic_number() {
		assert_eq!(MAGIC_MAINNET, Network::Mainnet.magic(&ConsensusFork::BitcoinCore));
		assert_eq!(MAGIC_TESTNET, Network::Testnet.magic(&ConsensusFork::BitcoinCore));
		assert_eq!(MAGIC_REGTEST, Network::Regtest.magic(&ConsensusFork::BitcoinCore));
		assert_eq!(MAGIC_UNITEST, Network::Unitest.magic(&ConsensusFork::BitcoinCore));
	}

	#[test]
	fn test_network_max_bits() {
		assert_eq!(Network::Mainnet.max_bits(&ConsensusFork::BitcoinCore), *MAX_BITS_MAINNET);
		assert_eq!(Network::Testnet.max_bits(&ConsensusFork::BitcoinCore), *MAX_BITS_TESTNET);
		assert_eq!(Network::Regtest.max_bits(&ConsensusFork::BitcoinCore), *MAX_BITS_REGTEST);
		assert_eq!(Network::Unitest.max_bits(&ConsensusFork::BitcoinCore), Compact::max_value().into());
	}

	#[test]
	fn test_network_port() {
		assert_eq!(Network::Mainnet.port(&ConsensusFork::BitcoinCore), 8333);
		assert_eq!(Network::Testnet.port(&ConsensusFork::BitcoinCore), 18333);
		assert_eq!(Network::Regtest.port(&ConsensusFork::BitcoinCore), 18444);
		assert_eq!(Network::Unitest.port(&ConsensusFork::BitcoinCore), 18444);
	}

	#[test]
	fn test_network_rpc_port() {
		assert_eq!(Network::Mainnet.rpc_port(), 8332);
		assert_eq!(Network::Testnet.rpc_port(), 18332);
		assert_eq!(Network::Regtest.rpc_port(), 18443);
		assert_eq!(Network::Unitest.rpc_port(), 18443);
	}
}
