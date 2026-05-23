# HumanPulse Protocol — Partner-Funded Verification Capacity Model

## Protocol Evolution Note for Pulse-Bronze Readiness

This document defines an active protocol evolution direction. It does not modify the frozen Solana Frontier Hackathon submission, the final whitepaper v7.2, pitch deck v9, or business plan v1.

HumanPulse Protocol is privacy-first Proof of Unique Humanity infrastructure for Solana.

> HumanPulse proves the presence of a unique living human at a specific moment in time — instantly, privately, and on-chain.

## 1. Purpose

HumanPulse is evolving beyond the frozen hackathon MVP and the current Claim Guard Devnet prototype toward a partner-funded verification capacity model.

The purpose of this document is to define the intended default economic and UX model for Pulse-Bronze readiness:

- the end user generally does not need SOL;
- the end user generally does not need HPP;
- the partner that benefits from Sybil resistance funds the verification context;
- HumanPulse abstracts Solana gas through sponsored infrastructure;
- HPP powers verification capacity, protocol economics, burn/routing, validator incentives, and treasury flows.

This document is a protocol evolution note. It does not claim that the current Devnet prototype already implements the full partner-funded model.

## 2. Core Model

The core principle is:

> The party that benefits from Sybil resistance funds the verification capacity.

In practice:

- a partner, DAO, company, application, institution, campaign, or protocol creates a HumanPulse context;
- that context defines the protected right, such as an airdrop claim, vote, allocation, gated access right, ticket, or reward;
- the partner funds or authorizes a context-specific HPP verification budget;
- the end user completes HumanPulse verification;
- the end user does not need to manually acquire SOL or HPP where a sponsored flow is enabled;
- HumanPulse relayer infrastructure abstracts SOL gas;
- Claim Guard or future HumanPulse modules consume, route, or burn HPP from the partner-funded context budget only when a valid human right is accepted, at least for the initial Bronze staged pilot.

This model preserves HPP as the protocol usage asset while removing token acquisition friction from ordinary end users.

## 3. Why End Users Should Not Need SOL or HPP

Requiring every end user to acquire HPP before verification creates friction.

For many HumanPulse use cases, the end user is not the economic buyer. The buyer is the application that wants to reduce bot activity, duplicate participation, fake accounts, or Sybil abuse.

Examples:

- an airdrop project wants one claim per eligible human;
- a DAO wants stronger human participation controls;
- a launchpad wants fairer allocation;
- a game wants to reduce bot farming;
- a social platform wants to limit fake account abuse;
- an enterprise wants a privacy-preserving human verification layer.

In these cases, the partner receives the primary value. Therefore, the partner should fund the verification capacity.

The user should experience HumanPulse as a verification flow, not as a token checkout flow.

## 4. Partner-Funded HPP Verification Capacity

The intended Pulse-Bronze readiness model introduces partner-funded verification capacity.

Key concepts:

- **context verification budget**: the amount of HPP or equivalent capacity allocated to a specific HumanPulse context;
- **sponsor-funded verification**: the partner funds the cost of valid accepted verifications;
- **partner HPP vault**: a future account or program-controlled structure holding partner-funded HPP capacity;
- **sponsor HPP token account**: a partner-controlled or program-authorized HPP account used to fund verification capacity;
- **context budget account**: a future on-chain account that tracks capacity, policy, and remaining verification rights for a context;
- **remaining verification capacity**: the number or value of accepted verifications still available for the context;
- **fee_source = partner_sponsor**: a future fee-source mode indicating that the partner, not the end user, funds HPP verification cost;
- **accepted-claim fee consumption**: HPP is consumed, routed, or burned when a valid human claim is accepted.

This model is designed to support real partner integrations without requiring each end user to hold HPP.

## 5. Example: 1,000 Human-Verified Airdrop Claims

A project wants to protect an airdrop with 1,000 eligible human-verified claims.

The intended model is:

1. The project creates a HumanPulse context:
   - `context_id = project_airdrop_round_1`
   - `right_type = airdrop_claim`
   - `required_assurance = bronze`
   - `required_age = over_18`, if required by policy
   - `max_claims = 1000`

2. The project funds 1,000 HPP or equivalent verification capacity.

3. Each end user clicks **Verify by HumanPulse**.

4. The end user completes the verification flow.

5. The end user does not need SOL.

6. The end user does not need HPP.

7. If the user passes the required checks and the nullifier has not been used in that context:
   - the claim is accepted;
   - one unit of HPP verification capacity is consumed from the partner-funded context budget;
   - the nullifier is recorded;
   - the user receives access to the protected right.

8. If the same human attempts to claim again in the same context:
   - the duplicate is rejected;
   - no new valid right is consumed.

For the initial Bronze staged pilot, the preferred model is to consume partner-funded HPP only when a valid claim is accepted.

## 6. Relationship to Claim Guard

Claim Guard is the first concrete HumanPulse module where this model should be implemented.

The current Claim Guard Devnet prototype demonstrates:

- context-specific nullifiers;
- sponsored SOL fees through a relayer;
- HPP proof consumption;
- duplicate rejection;
- a browser-based Bronze verification flow;
- Solana Devnet execution.

However, the current Devnet prototype may still contain user-side HPP token-account assumptions. It should not be represented as the final partner-funded model.

The migration path is:

- preserve the working Devnet prototype;
- introduce policy-scoped relayer architecture;
- introduce partner-funded context budgets;
- migrate HPP fee source from user-funded assumptions to partner-funded capacity;
- preserve optional user-paid HPP only as a fallback or consumer model.

## 7. Relationship to Future HumanPulse Modules

The partner-funded capacity model applies to the entire HumanPulse Protocol, not only Claim Guard.

Future modules may include:

- **Vote Guard**: DAOs fund human voting eligibility or one-human participation controls;
- **Age Gate**: platforms fund threshold-based eligibility checks where appropriate;
- **Access Guard**: applications fund human-only access flows;
- **Launchpad Guard**: token launch platforms fund human allocation protection;
- **Gaming / Rewards Guard**: games fund anti-bot reward verification;
- **Social / Anti-Bot Verification**: platforms fund human presence checks for account integrity;
- **Enterprise Verification**: enterprises use prepaid capacity or partner-funded HPP budgets;
- **Public-sector / Civic Contexts**: institutions, foundations, or public programs fund access to protected human rights.

The shared principle remains:

> The integrating context funds the verification capacity; the human completes the verification.

## 8. $HPP Token Role

HPP remains the usage-driven protocol token.

The partner-funded model does not remove HPP from the system. It changes where HPP is used in the UX.

Instead of requiring each end user to buy HPP before verification, HPP powers:

- context verification budgets;
- partner-funded verification capacity;
- proof-consumption accounting;
- burn and routing flows;
- validator incentives;
- treasury flows;
- gas abstraction economics;
- protocol sustainability.

This moves HPP from end-user checkout friction to partner-funded protocol capacity.

## 9. User Trading / Holding / Liquidity Boundary

Users may still buy, hold, or trade HPP on DEX/CEX where available.

However:

- buying HPP should not be required for ordinary end-user verification in partner-funded contexts;
- holding HPP is separate from completing a sponsored HumanPulse verification;
- trading or holding HPP is a token-market activity, not the default verification UX;
- optional user-paid verification may exist as a fallback or consumer model, but it should not be the primary adoption path.

This distinction is important for usability.

The end user should be able to verify because a partner has funded the context, not because the end user has learned how to acquire and manage HPP.

## 10. Anti-Whale and Anti-Pump-and-Dump Protections

HPP tokenomics should remain careful, usage-driven, and resistant to unhealthy speculative dynamics.

Relevant protection areas include:

- vesting;
- liquidity protections;
- dynamic anti-whale tax / burn design;
- DAO governance;
- supply discipline;
- careful exchange and liquidity strategy;
- partner demand driven by real verification usage rather than hype.

These protections should be described carefully as tokenomics design and roadmap elements unless and until they are implemented, audited, and governed.

The protocol should avoid positioning HPP as a pure speculative token. HPP should be tied to verification capacity, usage, partner demand, and protocol economics.

## 11. Fee Source Models

HumanPulse should support multiple fee-source models, with partner-funded capacity as the default target.

### Model A — Partner-Sponsored HPP Capacity

Default target model.

The partner funds a context-specific HPP budget. End users complete verification without needing SOL or HPP.

Best for:

- airdrops;
- launchpads;
- DAOs;
- gaming rewards;
- gated access;
- enterprise campaigns;
- social anti-bot flows.

### Model B — Enterprise Prepaid Credits

The enterprise buys verification capacity through a commercial agreement.

The enterprise may pay in fiat, stablecoins, or HPP depending on the product model. HumanPulse may account for this capacity internally while preserving HPP as the protocol usage asset.

Best for:

- enterprise integrations;
- institutional partners;
- regulated or semi-regulated contexts requiring commercial billing;
- large-volume partner onboarding.

### Model C — DAO / Treasury-Sponsored Verification

A DAO treasury or ecosystem fund sponsors human verification for a governance, grant, allocation, or participation context.

Best for:

- DAO voting;
- governance access;
- grants;
- community allocation events;
- quadratic funding or public goods rounds.

### Model D — Optional User-Paid HPP

Optional fallback or consumer model.

A user may pay HPP directly where no partner sponsor exists, but this should not be the default UX for ordinary partner-funded verification.

Best for:

- consumer self-verification contexts;
- optional premium verification;
- fallback flows;
- future use cases where user payment is appropriate.

## 12. Technical Migration Path

The technical roadmap should introduce partner-funded fee-source concepts gradually.

Future code and protocol concepts may include:

- `partner_hpp_vault`;
- `sponsor_hpp_token_account`;
- `context_budget_account`;
- `remaining_verification_capacity`;
- `fee_source = partner_sponsor`;
- accepted-verification fee consumption;
- relayer policy registry;
- signed context policies;
- budget caps;
- per-context limits;
- emergency pause;
- monitoring and abuse controls;
- partner authority;
- context lifecycle management.

The first rule for migration is to preserve the working Devnet prototype while introducing new boundaries incrementally.

A safe migration path is:

1. document the partner-funded model at the protocol level;
2. reflect the model in active Claim Guard documentation;
3. extend relayer policy boundaries;
4. define context budget account semantics;
5. define sponsor HPP token account semantics;
6. introduce fee-source modes;
7. test partner-funded flow on Devnet;
8. only then evaluate staged Mainnet pilot readiness.

## 13. Privacy and UX Implications

Partner-funded verification improves UX because the user does not need to acquire SOL or HPP before verification.

It also supports the privacy-first HumanPulse model because the application can request a minimal proof outcome instead of collecting broad identity data.

The UX target is:

- click **Verify by HumanPulse**;
- complete the required local verification flow;
- sign where required for wallet/context binding;
- receive accepted, rejected, or duplicate status;
- avoid manual SOL or HPP acquisition in sponsored contexts.

The protocol should still preserve clear privacy boundaries:

- no raw biometric data on-chain;
- no raw document data on-chain;
- context-specific nullifiers;
- minimal proof-consumption records;
- local processing where technically supported;
- clear disclosure of Bronze limitations.

## 14. Business Model Implications

HumanPulse sells verification capacity and Sybil-resistance infrastructure to the party that receives value from it.

This is stronger than requiring every end user to become a token buyer before verification.

The partner-funded model supports:

- easier onboarding;
- clearer enterprise sales;
- smoother DAO and launchpad adoption;
- better user experience;
- stronger protocol usage demand;
- predictable context-level budgets;
- campaign-level accounting;
- recurring verification demand.

In this model, HPP demand is connected to usage by partners and applications, not only to speculative trading.

## 15. What Remains Historical / Frozen

The following remain historical / frozen Solana Frontier Hackathon materials:

- HumanPulse Technical Whitepaper v7.2;
- HumanPulse Business Plan v1;
- Solana Frontier pitch deck v9;
- hackathon narrative artifacts;
- submitted MVP materials;
- original demo packaging;
- historical Devnet proof-consumption examples.

They should not be retroactively edited to imply that the partner-funded model was part of the original submission.

Future versions may be created separately, such as:

- HumanPulse Technical Whitepaper v8;
- HumanPulse Business Plan v2;
- Pitch Deck v10;
- updated protocol roadmap;
- updated product documentation.

## 16. Non-Goals

This document does not claim:

- current Mainnet readiness;
- audited implementation;
- legal compliance guarantee;
- replacement for KYC;
- fully solved age verification;
- that optional research modules are implemented;
- that the current Claim Guard Devnet prototype already implements full partner-funded HPP capacity;
- that users cannot buy, hold, or trade HPP;
- that all tokenomics protections are already implemented.

This document defines an active protocol direction and migration target.

## 17. Next Implementation Blocks

Recommended next blocks:

1. Link this model from active READMEs.
2. Reflect this model in ClaimGuardDevnet active docs.
3. Update future Business Plan v2 / Whitepaper v8 / Pitch Deck v10.
4. Update the landing website only after public message review.
5. Implement partner-funded fee source in the Claim Guard roadmap.
6. Extend relayer policy registry for context budgets.
7. Define partner HPP vault / context budget account model.
8. Preserve optional user-paid HPP as fallback, not default.
9. Define sponsor-funded verification status and accounting.
10. Define partner integration copy for “Verify by HumanPulse”.

Each block should clearly separate frozen hackathon materials, current Devnet prototype behavior, Pulse-Bronze readiness, technical roadmap, and optional research modules.
