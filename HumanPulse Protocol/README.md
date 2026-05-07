# HumanPulse Protocol ($HPP)

**Privacy-first Proof of Unique Humanity infrastructure for Solana.**

HumanPulse proves the presence of a unique living human at a specific moment in time — instantly, privately, and on-chain.

---

## Overview

HumanPulse Protocol is designed to provide Sybil-resistant proof of unique human presence for on-chain applications.

The current Solana Frontier Hackathon MVP focuses on a practical, lightweight implementation for:

- DAO voting
- airdrops
- token launches
- anti-bot gating
- proof-of-personhood access flows

The MVP combines webcam-based liveness, challenge-response verification, a Solana Devnet token, and an Anchor burn-on-use program.

---

## Official Links

- **Live Website:** https://humanpulse-landing.vercel.app/
- **Demo Video:** https://humanpulse-landing.vercel.app/HumanPulse-demo.mp4
- **Solana Devnet Program:** https://explorer.solana.com/address/D9tRQi8nZzARTJZXTmSsy4hrx4BwKUNPezKzayvQ5B5N?cluster=devnet
- **$HPP Token on Solana Devnet:** https://explorer.solana.com/address/EsBZKfFmhVAypmVm6QmenevZ56XZQZUjYF8CLzVnZSnR?cluster=devnet
- **Landing Repository:** https://github.com/humanpulseprotocol/humanpulse-landing
- **Technical Whitepaper:** https://github.com/humanpulseprotocol/HumanPulse-Protocol/blob/main/HumanPulse%20Protocol/whitepaper/HumanPulse_Technical_Whitepaper_v7.2_FINAL.pdf

---

## What We Built for the Hackathon

The current MVP / hackathon implementation includes:

- webcam-based liveness flow
- challenge-response verification
- demo website / widget
- Solana Devnet $HPP token
- Anchor burn-on-use program
- on-chain proof interaction on Solana Devnet
- public demo video
- technical whitepaper v7.2
- pitch deck for Solana Frontier Hackathon

This implementation is intended to demonstrate how a privacy-preserving human verification layer can be integrated into Solana-native applications.

---

## Problem

The internet cannot reliably distinguish humans from automated agents at scale.

This creates major issues for:

- airdrop farming
- DAO governance manipulation
- bot-driven token launches
- fake account creation
- identity fraud
- AI-agent abuse

Traditional solutions such as passwords, CAPTCHAs, and centralized KYC introduce friction, privacy risk, or weak Sybil resistance.

HumanPulse is designed to significantly increase the cost of Sybil attacks while minimizing the amount of personal data required.

---

## Solution

HumanPulse provides a lightweight Proof of Unique Humanity flow.

At a high level, the protocol verifies:

1. that a live human is present
2. that the human responds to an unpredictable challenge
3. that the verification can be consumed by an on-chain application
4. that the proof interaction can occur without storing raw biometric data on-chain

The MVP demonstrates this with webcam-based liveness, challenge-response verification, and a Solana Devnet burn-on-use mechanism.

---

## Architecture

The HumanPulse architecture is structured around three layers:

1. **Liveness Detection**  
   Detects signs of live human presence through webcam-based interaction.

2. **Challenge-Response Verification**  
   Adds unpredictability through user-facing real-time challenges.

3. **On-Chain Proof Consumption**  
   Uses Solana Devnet and an Anchor program to demonstrate proof usage through a burn-on-use mechanism.

The broader technical roadmap explores more advanced privacy-preserving proof systems and aggregation mechanisms.

---

## Solana Integration

The hackathon MVP integrates with Solana through:

- a Devnet $HPP token
- an Anchor burn-on-use program
- on-chain verification consumption
- low-cost, high-throughput proof interactions
- compatibility with DAO, airdrop, and token launch use cases

### Devnet Program

https://explorer.solana.com/address/D9tRQi8nZzARTJZXTmSsy4hrx4BwKUNPezKzayvQ5B5N?cluster=devnet

### $HPP Token on Solana Devnet

https://explorer.solana.com/address/EsBZKfFmhVAypmVm6QmenevZ56XZQZUjYF8CLzVnZSnR?cluster=devnet

---

## Privacy Positioning

HumanPulse is designed to minimize data exposure.

The protocol direction is privacy-first:

- no raw biometric data is intended to be stored on-chain
- verification is designed around proof consumption, not identity disclosure
- the MVP demonstrates proof interaction rather than centralized identity storage
- future research explores local proof generation, zero-knowledge aggregation, and advanced privacy-preserving modules

HumanPulse should not be understood as a KYC provider. It is designed as a Proof of Unique Humanity infrastructure layer.

---

## MVP vs Roadmap vs Research Modules

### Current MVP / Hackathon Implementation

- Sybil resistance for DAOs, airdrops, and token launches
- webcam-based liveness
- challenge-response verification
- Solana Devnet $HPP token
- Anchor burn-on-use program
- demo website / widget
- public demo video

### Technical Roadmap

- stronger proof aggregation
- validator / aggregator architecture
- expanded Solana integration
- privacy-preserving proof flows
- production-oriented protocol hardening
- mainnet-readiness path

### Optional Research Modules

The following modules are part of the research roadmap and are not required for the MVP:

- recursive zk-STARK aggregation
- TEE-based local proof generation
- LiDAR photonics
- pupillary reflex verification
- micro-expression analysis
- multi-device verification

---

## Use Cases

HumanPulse is designed for applications that need Sybil-resistant human presence without relying on centralized identity collection.

Primary use cases include:

- DAO voting
- airdrop eligibility
- token launch protection
- anti-bot access gates
- quadratic funding
- decentralized social applications
- proof-of-personhood gated experiences

---

## Demo

Live website:

https://humanpulse-landing.vercel.app/

Demo video:

https://humanpulse-landing.vercel.app/HumanPulse-demo.mp4

---

## Repository Structure

This repository contains the HumanPulse Protocol hackathon implementation and supporting materials.

Depending on the branch and project state, it may include:

- Solana / Anchor program files
- frontend demo files
- protocol documentation
- whitepaper references
- assets and diagrams
- hackathon packaging materials

---

## Technical Whitepaper

The final technical reference is:

**HumanPulse Protocol Technical Whitepaper v7.2**

The whitepaper is the source of truth for the protocol architecture, privacy model, token design, and long-term roadmap.

---

## Status

HumanPulse Protocol is currently in MVP / hackathon implementation stage.

The current implementation is intended to demonstrate feasibility, user flow, Solana integration, and the core protocol narrative. It should not be represented as a fully production-hardened mainnet deployment.

---

## License

This project is licensed under the terms described in the repository LICENSE file.
