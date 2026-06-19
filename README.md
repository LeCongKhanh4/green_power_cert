# green_power_cert

## Project Title
green_power_cert

## Project Description
Renewable Energy Certificates (RECs) are the off-chain bookkeeping tools
used today to prove that a certain amount of clean energy (measured in
kWh) was generated from a renewable source (solar, wind, hydro, biomass,
...). The problem is that this market is fragmented across registries,
auditors and PDFs, which makes RECs hard to trade, easy to double count,
and slow to settle. **green_power_cert** brings the whole REC lifecycle
onto a public, append-only blockchain: a certified issuer mints a REC,
traders transfer it peer-to-peer, and the final buyer retires it to
claim the underlying green kWh. Soroban on Stellar gives us low fees,
fast finality and native `require_auth` signatures, which is all we need
to make every REC tamper-proof and globally verifiable.

## Project Vision
Our long-term vision is a transparent, permissionless registry of
green-energy claims that any company, NGO or individual can audit in
real time. By putting REC issuance, transfer, retirement and revocation
on Stellar, we want to remove the manual reconciliation overhead of
traditional certificate registries, eliminate double counting, and let
small renewable producers monetize their kWh directly to global buyers
without an intermediary. Eventually the contract should be a building
block that other dApps (carbon markets, ESG dashboards, supply-chain
trackers) can compose on top of.

## Key Features
- **Mint** — an authorized `certifier` mints a REC tied to a renewable
  `source`, an amount of `kwh` and a production `period`. The certifier
  is recorded as the original issuer and initial owner.
- **Transfer** — the current owner of a valid REC can transfer it to
  any other Stellar address in a single on-chain transaction.
- **Retire** — the owner of a valid REC can permanently retire it,
  recording a free-form reason (e.g. `scope-2-offset-2025`). Retired
  RECs cannot be resold, which prevents double claiming.
- **Revoke** — only the original `certifier` can revoke a REC they
  minted, for example when a meter reading is later proven to be
  invalid. A reason is stored on-chain for transparency.
- **Verify** — anyone can query a `cert_id` and get back its status
  (`0` = unknown, `1` = valid, `2` = retired, `3` = revoked) without
  needing a wallet or signature.
- **is_retired** — a cheap helper that returns `true` only if the REC
  exists and has been retired, useful for ESG dashboards and audits.

## Contract

- **Network:** Stellar Testnet (Public)
- **Scope:** environment dApp — see `contracts/green_power_cert/src/lib.rs` for the full green_power_cert business logic.
- **Functions exposed:** see `Key Features` above and the `pub fn` list in `lib.rs`.
- **Contract ID:** `CDYLLO7VF2N7WMNSP26SUUE263NDOJ7KEEU3PLVKGCO4RNVYMQAVIMHM`
- **Explorer template:** `https://stellar.expert/explorer/testnet/tx/1d6d17260c977a478329f609a8be6bee0839b95f6f9a18f0ac07900c140818d5`


## Future Scope
- **Batch minting** — allow a certifier to mint many RECs in a single
  transaction, useful for utility-scale producers.
- **Fractional RECs** — split a single REC into smaller tradable units
  so that retail buyers can offset part of their electricity usage.
- **Oracle-driven source verification** — plug in a Soroban oracle
  that reads real IoT meter data before allowing a `mint`.
- **On-chain marketplace** — add `list_for_sale` / `buy` primitives and
  a stablecoin-denominated order book, so RECs can be traded
  peer-to-peer without an off-platform escrow.
- **Aggregated retirement proofs** — allow an address to retire many
  RECs at once and emit a single signed "offset certificate" event
  for ESG reporting.
- **Cross-chain bridging** — mirror selected REC events to other
  chains (Ethereum, Polygon) so that traditional carbon markets can
  consume Stellar-issued certificates.
- **Audit / analytics views** — add read-only helpers such as
  `total_issued_kwh`, `total_retired_kwh` and per-source breakdowns to
  power public dashboards.

## Profile

- **Name:** <!-- Fill github name -->
- **Project:** `green_power_cert` (environment)
- **Built with:** Soroban SDK 25, Rust, Stellar Testnet
