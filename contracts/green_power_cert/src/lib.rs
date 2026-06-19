#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol};

// Status codes returned by `verify`.
// 0 = none/unknown, 1 = valid, 2 = retired, 3 = revoked.
const STATUS_VALID: u32 = 1;
const STATUS_RETIRED: u32 = 2;
const STATUS_REVOKED: u32 = 3;

/// Renewable Energy Certificate (REC) data model stored on-chain.
#[contracttype]
#[derive(Clone)]
pub struct Rec {
    pub certifier: Address,
    pub owner: Address,
    pub source: Symbol,        // e.g. "solar", "wind", "hydro", "biomass"
    pub kwh: u64,              // amount of clean energy represented (kWh)
    pub period: Symbol,        // production period, e.g. "2025-Q1"
    pub status: u32,           // 1=valid, 2=retired, 3=revoked
    pub retire_reason: Symbol, // reason recorded when retired
    pub revoke_reason: Symbol, // reason recorded when revoked
}

/// GreenPowerCert: a Soroban smart contract for issuing, transferring,
/// retiring and revoking Renewable Energy Certificates (RECs) on Stellar.
#[contract]
pub struct GreenPowerCert;

#[contractimpl]
impl GreenPowerCert {
    /// Mint a new Renewable Energy Certificate (REC).
    ///
    /// The `certifier` (an authorized issuer such as a grid operator or
    /// auditor) signs the transaction, defines the renewable `source`
    /// (e.g. solar, wind), the `kwh` amount and the production `period`.
    /// On mint, the certifier is set as the initial owner.
    pub fn mint(
        env: Env,
        certifier: Address,
        cert_id: u64,
        source: Symbol,
        kwh: u64,
        period: Symbol,
    ) {
        // Only the certifier can sign the mint.
        certifier.require_auth();

        if kwh == 0 {
            panic!("kwh must be greater than zero");
        }

        let key = (Symbol::new(&env, "cert"), cert_id);
        if env.storage().instance().has(&key) {
            panic!("cert already exists");
        }

        let rec = Rec {
            certifier: certifier.clone(),
            owner: certifier,
            source,
            kwh,
            period,
            status: STATUS_VALID,
            retire_reason: Symbol::new(&env, "none"),
            revoke_reason: Symbol::new(&env, "none"),
        };

        env.storage().instance().set(&key, &rec);
    }

    /// Transfer a valid REC from the current owner to a new owner.
    ///
    /// Useful for marketplaces where RECs are traded between
    /// producers, traders and end-buyers.
    pub fn transfer(env: Env, owner: Address, cert_id: u64, new_owner: Address) {
        // The current owner must authorize the transfer.
        owner.require_auth();

        let key = (Symbol::new(&env, "cert"), cert_id);
        let mut rec: Rec = env
            .storage()
            .instance()
            .get(&key)
            .expect("cert not found");

        if rec.status != STATUS_VALID {
            panic!("cert is not transferable");
        }
        if rec.owner != owner {
            panic!("caller is not the owner");
        }
        if new_owner == owner {
            panic!("new_owner is the same as current owner");
        }

        rec.owner = new_owner;
        env.storage().instance().set(&key, &rec);
    }

    /// Retire a REC on behalf of its owner.
    ///
    /// Retirement permanently marks the REC as "claimed" so its
    /// underlying kWh cannot be resold or double-counted. The owner
    /// records a free-form `reason` (e.g. "scope-2-offset-2025").
    pub fn retire(env: Env, owner: Address, cert_id: u64, reason: Symbol) {
        owner.require_auth();

        let key = (Symbol::new(&env, "cert"), cert_id);
        let mut rec: Rec = env
            .storage()
            .instance()
            .get(&key)
            .expect("cert not found");

        if rec.status != STATUS_VALID {
            panic!("only valid certs can be retired");
        }
        if rec.owner != owner {
            panic!("caller is not the owner");
        }

        rec.status = STATUS_RETIRED;
        rec.retire_reason = reason;
        env.storage().instance().set(&key, &rec);
    }

    /// Revoke a REC. Only the original certifier can revoke a certificate
    /// they minted (e.g. when a measurement is later found to be invalid).
    pub fn revoke(env: Env, certifier: Address, cert_id: u64, reason: Symbol) {
        certifier.require_auth();

        let key = (Symbol::new(&env, "cert"), cert_id);
        let mut rec: Rec = env
            .storage()
            .instance()
            .get(&key)
            .expect("cert not found");

        if rec.certifier != certifier {
            panic!("caller is not the original certifier");
        }
        if rec.status != STATUS_VALID {
            panic!("only valid certs can be revoked");
        }

        rec.status = STATUS_REVOKED;
        rec.revoke_reason = reason;
        env.storage().instance().set(&key, &rec);
    }

    /// Return the on-chain status of a REC.
    ///
    /// * `0` — no certificate with this id exists
    /// * `1` — valid (active, can be transferred or retired)
    /// * `2` — retired (claimed, kWh consumed)
    /// * `3` — revoked (invalidated by the certifier)
    pub fn verify(env: Env, cert_id: u64) -> u32 {
        let key = (Symbol::new(&env, "cert"), cert_id);
        match env.storage().instance().get::<_, Rec>(&key) {
            Some(rec) => rec.status,
            None => 0u32,
        }
    }

    /// Quick helper: returns `true` only if the certificate exists and
    /// is currently in the `retired` state.
    pub fn is_retired(env: Env, cert_id: u64) -> bool {
        let key = (Symbol::new(&env, "cert"), cert_id);
        match env.storage().instance().get::<_, Rec>(&key) {
            Some(rec) => rec.status == STATUS_RETIRED,
            None => false,
        }
    }
}
