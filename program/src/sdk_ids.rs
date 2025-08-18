pub mod address_lookup_table {
    super::super::declare_id!("AddressLookupTab1e1111111111111111111111111");
}

pub mod bpf_loader {
    super::super::declare_id!("BPFLoader2111111111111111111111111111111111");
}

pub mod bpf_loader_deprecated {
    super::super::declare_id!("BPFLoader1111111111111111111111111111111111");
}

pub mod bpf_loader_upgradeable {
    super::super::declare_id!("BPFLoaderUpgradeab1e11111111111111111111111");
}

pub mod compute_budget {
    super::super::declare_id!("ComputeBudget111111111111111111111111111111");
}

pub mod config {
    super::super::declare_id!("Config1111111111111111111111111111111111111");
}

pub mod ed25519_program {
    super::super::declare_id!("Ed25519SigVerify111111111111111111111111111");
}

pub mod feature {
    super::super::declare_id!("Feature111111111111111111111111111111111111");
}

/// A designated address for burning lamports.
///
/// Lamports credited to this address will be removed from the total supply
/// (burned) at the end of the current block.
pub mod incinerator {
    super::super::declare_id!("1nc1nerator11111111111111111111111111111111");
}

pub mod loader_v4 {
    super::super::declare_id!("LoaderV411111111111111111111111111111111111");
}

pub mod native_loader {
    super::super::declare_id!("NativeLoader1111111111111111111111111111111");
}

pub mod secp256k1_program {
    super::super::declare_id!("KeccakSecp256k11111111111111111111111111111");
}

pub mod secp256r1_program {
    super::super::declare_id!("Secp256r1SigVerify1111111111111111111111111");
}

pub mod stake {
    pub mod config {
        super::super::super::declare_deprecated_id!("StakeConfig11111111111111111111111111111111");
    }
    super::super::declare_id!("Stake11111111111111111111111111111111111111");
}

pub mod system_program {
    super::super::declare_id!("11111111111111111111111111111111");
}

pub mod vote {
    super::super::declare_id!("Vote111111111111111111111111111111111111111");
}

pub mod sysvar {
    // Owner pubkey for sysvar accounts
    super::super::declare_id!("Sysvar1111111111111111111111111111111111111");
    pub mod clock {
        super::super::super::declare_id!("SysvarC1ock11111111111111111111111111111111");
    }
    pub mod epoch_rewards {
        super::super::super::declare_id!("SysvarEpochRewards1111111111111111111111111");
    }
    pub mod epoch_schedule {
        super::super::super::declare_id!("SysvarEpochSchedu1e111111111111111111111111");
    }
    pub mod fees {
        super::super::super::declare_id!("SysvarFees111111111111111111111111111111111");
    }
    pub mod instructions {
        super::super::super::declare_id!("Sysvar1nstructions1111111111111111111111111");
    }
    pub mod last_restart_slot {
        super::super::super::declare_id!("SysvarLastRestartS1ot1111111111111111111111");
    }
    pub mod recent_blockhashes {
        super::super::super::declare_id!("SysvarRecentB1ockHashes11111111111111111111");
    }
    pub mod rent {
        super::super::super::declare_id!("SysvarRent111111111111111111111111111111111");
    }
    pub mod rewards {
        super::super::super::declare_id!("SysvarRewards111111111111111111111111111111");
    }
    pub mod slot_hashes {
        super::super::super::declare_id!("SysvarS1otHashes111111111111111111111111111");
    }
    pub mod slot_history {
        super::super::super::declare_id!("SysvarS1otHistory11111111111111111111111111");
    }
    pub mod stake_history {
        super::super::super::declare_id!("SysvarStakeHistory1111111111111111111111111");
    }
}

pub mod zk_token_proof_program {
    super::super::declare_id!("ZkTokenProof1111111111111111111111111111111");
}

pub mod zk_elgamal_proof_program {
    super::super::declare_id!("ZkE1Gama1Proof11111111111111111111111111111");
}
