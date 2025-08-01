//! Definitions related to Solana over QUIC.
use {solana_keypair::Keypair, std::time::Duration};

pub const QUIC_PORT_OFFSET: u16 = 6;
// Empirically found max number of concurrent streams
// that seems to maximize TPS on GCE (higher values don't seem to
// give significant improvement or seem to impact stability)
pub const QUIC_MAX_UNSTAKED_CONCURRENT_STREAMS: usize = 128;
pub const QUIC_MIN_STAKED_CONCURRENT_STREAMS: usize = 128;

pub const QUIC_TOTAL_STAKED_CONCURRENT_STREAMS: usize = 100_000;

// Set the maximum concurrent stream numbers to avoid excessive streams.
// The value was lowered from 2048 to reduce contention of the limited
// receive_window among the streams which is observed in CI bench-tests with
// forwarded packets from staked nodes.
pub const QUIC_MAX_STAKED_CONCURRENT_STREAMS: usize = 512;

/// QUIC connection idle timeout. The connection will be closed if
/// there are no activities on it within the timeout window.
pub const QUIC_MAX_TIMEOUT: Duration = Duration::from_secs(60);

/// To avoid idle timeout, the QUIC endpoint sends a ping every
/// QUIC_KEEP_ALIVE. This shouldn't be too low to avoid unnecessary ping traffic.
pub const QUIC_KEEP_ALIVE: Duration = Duration::from_secs(45);

// Disable Quic send fairness.
// When set to false, streams are still scheduled based on priority,
// but once a chunk of a stream has been written out, quinn tries to complete
// the stream instead of trying to round-robin balance it among the streams
// with the same priority.
// See https://github.com/quinn-rs/quinn/pull/2002.
pub const QUIC_SEND_FAIRNESS: bool = false;

// Based on commonly-used handshake timeouts for various TCP
// applications. Different applications vary, but most seem to
// be in the 30-60 second range
pub const QUIC_CONNECTION_HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(60);

/// The receive window for QUIC connection from unstaked nodes is
/// set to this ratio times [`solana_packet::PACKET_DATA_SIZE`]
///
/// [`solana_packet::PACKET_DATA_SIZE`]: https://docs.rs/solana-packet/latest/solana_packet/constant.PACKET_DATA_SIZE.html
pub const QUIC_UNSTAKED_RECEIVE_WINDOW_RATIO: u64 = 128;

/// The receive window for QUIC connection from minimum staked nodes is
/// set to this ratio times [`solana_packet::PACKET_DATA_SIZE`]
///
/// [`solana_packet::PACKET_DATA_SIZE`]: https://docs.rs/solana-packet/latest/solana_packet/constant.PACKET_DATA_SIZE.html
pub const QUIC_MIN_STAKED_RECEIVE_WINDOW_RATIO: u64 = 128;

/// The receive window for QUIC connection from maximum staked nodes is
/// set to this ratio times [`solana_packet::PACKET_DATA_SIZE`]
///
/// [`solana_packet::PACKET_DATA_SIZE`]: https://docs.rs/solana-packet/latest/solana_packet/constant.PACKET_DATA_SIZE.html
pub const QUIC_MAX_STAKED_RECEIVE_WINDOW_RATIO: u64 = 512;

pub trait NotifyKeyUpdate {
    fn update_key(&self, key: &Keypair) -> Result<(), Box<dyn core::error::Error>>;
}
