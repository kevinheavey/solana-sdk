//! The definition of a Solana network packet.
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use {
    bitflags::bitflags,
    std::{
        fmt,
        net::{IpAddr, Ipv4Addr, SocketAddr},
        slice::SliceIndex,
    },
};
/// Maximum over-the-wire size of a Transaction
///   1280 is IPv6 minimum MTU
///   40 bytes is the size of the IPv6 header
///   8 bytes is the size of the fragment header
pub const PACKET_DATA_SIZE: usize = 1280 - 40 - 8;

bitflags! {
    #[repr(C)]
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct PacketFlags: u8 {
        const DISCARD        = 0b0000_0001;
        const FORWARDED      = 0b0000_0010;
        const REPAIR         = 0b0000_0100;
        const SIMPLE_VOTE_TX = 0b0000_1000;
        // Previously used - this can now be re-used for something else.
        const UNUSED_0  = 0b0001_0000;
        // Previously used - this can now be re-used for something else.
        const UNUSED_1 = 0b0010_0000;
        /// For tracking performance
        const PERF_TRACK_PACKET  = 0b0100_0000;
        /// For marking packets from staked nodes
        const FROM_STAKED_NODE = 0b1000_0000;
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Meta {
    pub size: usize,
    pub addr: IpAddr,
    pub port: u16,
    pub flags: PacketFlags,
}

// serde_as is used as a work around because array isn't supported by serde
// (and serde_bytes).
//
// the root cause is of a historical special handling for [T; 0] in rust's
// `Default` and supposedly mirrored serde's `Serialize` (macro) impls,
// pre-dating stabilized const generics, meaning it'll take long time...:
//   https://github.com/rust-lang/rust/issues/61415
//   https://github.com/rust-lang/rust/issues/88744#issuecomment-1138678928
//
// Due to the nature of the root cause, the current situation is complicated.
// All in all, the serde_as solution is chosen for good perf and low maintenance
// need at the cost of another crate dependency..
//
// For details, please refer to the below various links...
//
// relevant merged/published pr for this serde_as functionality used here:
//   https://github.com/jonasbb/serde_with/pull/277
// open pr at serde_bytes:
//   https://github.com/serde-rs/bytes/pull/28
// open issue at serde:
//   https://github.com/serde-rs/serde/issues/1937
// closed pr at serde (due to the above mentioned [N; 0] issue):
//   https://github.com/serde-rs/serde/pull/1860
// ryoqun's dirty experiments:
//   https://github.com/ryoqun/serde-array-comparisons
//
// We use the cfg_eval crate as advised by the serde_with guide:
// https://docs.rs/serde_with/latest/serde_with/guide/serde_as/index.html#gating-serde_as-on-features
#[repr(C)]
pub struct Packet {
    // Bytes past Packet.meta.size are not valid to read from.
    // Use Packet.data(index) to read from the buffer.
    buffer: [u8; PACKET_DATA_SIZE],
    meta: Meta,
}

impl Packet {
    pub fn new(buffer: [u8; PACKET_DATA_SIZE], meta: Meta) -> Self {
        Self { buffer, meta }
    }

    /// Returns an immutable reference to the underlying buffer up to
    /// packet.meta.size. The rest of the buffer is not valid to read from.
    /// packet.data(..) returns packet.buffer.get(..packet.meta.size).
    /// Returns None if the index is invalid or if the packet is already marked
    /// as discard.
    #[inline]
    pub fn data<I>(&self, index: I) -> Option<&<I as SliceIndex<[u8]>>::Output>
    where
        I: SliceIndex<[u8]>,
    {
        // If the packet is marked as discard, it is either invalid or
        // otherwise should be ignored, and so the payload should not be read
        // from.
        if self.meta.discard() {
            None
        } else {
            self.buffer.get(..self.meta.size)?.get(index)
        }
    }

    /// Returns a mutable reference to the entirety of the underlying buffer to
    /// write into. The caller is responsible for updating Packet.meta.size
    /// after writing to the buffer.
    #[inline]
    pub fn buffer_mut(&mut self) -> &mut [u8] {
        debug_assert!(!self.meta.discard());
        &mut self.buffer[..]
    }

    #[inline]
    pub fn meta(&self) -> &Meta {
        &self.meta
    }

    #[inline]
    pub fn meta_mut(&mut self) -> &mut Meta {
        &mut self.meta
    }
}

impl fmt::Debug for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Packet {{ size: {:?}, addr: {:?} }}",
            self.meta.size,
            self.meta.socket_addr()
        )
    }
}

#[allow(clippy::uninit_assumed_init)]
impl Default for Packet {
    fn default() -> Self {
        let buffer = std::mem::MaybeUninit::<[u8; PACKET_DATA_SIZE]>::uninit();
        Self {
            buffer: unsafe { buffer.assume_init() },
            meta: Meta::default(),
        }
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.meta() == other.meta() && self.data(..) == other.data(..)
    }
}

impl Meta {
    pub fn socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.addr, self.port)
    }

    pub fn set_socket_addr(&mut self, socket_addr: &SocketAddr) {
        self.addr = socket_addr.ip();
        self.port = socket_addr.port();
    }

    pub fn set_from_staked_node(&mut self, from_staked_node: bool) {
        self.flags
            .set(PacketFlags::FROM_STAKED_NODE, from_staked_node);
    }

    #[inline]
    pub fn discard(&self) -> bool {
        self.flags.contains(PacketFlags::DISCARD)
    }

    #[inline]
    pub fn set_discard(&mut self, discard: bool) {
        self.flags.set(PacketFlags::DISCARD, discard);
    }

    #[inline]
    pub fn set_track_performance(&mut self, is_performance_track: bool) {
        self.flags
            .set(PacketFlags::PERF_TRACK_PACKET, is_performance_track);
    }

    #[inline]
    pub fn set_simple_vote(&mut self, is_simple_vote: bool) {
        self.flags.set(PacketFlags::SIMPLE_VOTE_TX, is_simple_vote);
    }

    #[inline]
    pub fn forwarded(&self) -> bool {
        self.flags.contains(PacketFlags::FORWARDED)
    }

    #[inline]
    pub fn repair(&self) -> bool {
        self.flags.contains(PacketFlags::REPAIR)
    }

    #[inline]
    pub fn is_simple_vote_tx(&self) -> bool {
        self.flags.contains(PacketFlags::SIMPLE_VOTE_TX)
    }

    #[inline]
    pub fn is_perf_track_packet(&self) -> bool {
        self.flags.contains(PacketFlags::PERF_TRACK_PACKET)
    }

    #[inline]
    pub fn is_from_staked_node(&self) -> bool {
        self.flags.contains(PacketFlags::FROM_STAKED_NODE)
    }
}

impl Default for Meta {
    fn default() -> Self {
        Self {
            size: 0,
            addr: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            port: 0,
            flags: PacketFlags::empty(),
        }
    }
}
