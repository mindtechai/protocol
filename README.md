# Gamverse Protocol
**Open-source Layer 1 for tokenized playtime on Polkadot**

## Overview
Gamverse turns **verified human playtime** into **fungible Time-Tokens (TT)** via:
- **PoP**: Proof-of-Play (entropy >0.2s reaction time)
- **UGID**: Universal Gamer ID (ZK cross-platform)
- **TT**: 1 TT = 60 minutes verified play

**Live on Kusama**: [Parachain ID 2000](https://kusama.subscan.io/parachain/2000)  
**100+ nodes** | **PoP pallet live** | **TT minting active**

## Quick Start
```bash
# 1. Clone
git clone https://github.com/mindtechai/protocol
cd protocol

# 2. Build
cargo build --release

# 3. Run local
./target/release/node-template --dev

# 4. Submit PoP (example)## Web2 Layer (Separate)
- Frontend: [gamverse/frontend](https://github.com/gamverse/frontend)
- API: [gamverse/api](https://github.com/gamverse/api)

> **Web2 frontends drive adoption. Web3 stays pure.**
pallet_pop::submit_pop(player: "0x123", entropy: 25)pallets/pop/        → PoP pallet (Rust)  
runtime/            → Substrate runtime + TT minting
scripts/deploy.sh   → Kusama collator deployment---
© 2025 Gamverse Labs OÜ. All rights reserved.  
Code licensed under MIT. Concepts protected by prior art.---
© 2025 Gamverse Labs OÜ.  
**Concepts (PoP, UGID, TT) protected by prior art — Nov 11, 2025.**
