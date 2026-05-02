# Contract: Architecture Readiness

## Purpose

Define the durable readiness artifact shape for architecture mode after bounded
clarification and critique.

## Required Sections

`readiness-assessment.md` MUST include these sections:

- `Summary`
- `Readiness Status`
- `Working Assumptions`
- `Unresolved Questions`
- `Blockers`
- `Accepted Risks`
- `Recommended Next Mode`

## Behavioral Requirements

- `Working Assumptions` MUST record only explicit temporary assumptions or
  defaults that currently bound the recommendation.
- `Working Assumptions` MUST NOT replace omitted canonical authored sections;
  missing authored sections remain missing-body signals.
- `Unresolved Questions` MUST keep remaining decision-changing uncertainty
  visible when the recommendation is still conditional.
- `Recommended Next Mode` MUST name an existing Canon mode and explain why that
  handoff is more trustworthy than treating the packet as architecture-ready.
- `Readiness Status` MUST state whether the packet is bounded but conditional,
  blocked, or ready for downstream consumption.

## Validation Responsibilities

- The architecture artifact contract owns the required sections.
- The architecture markdown renderer owns materializing the readiness shape.
- Focused contract and run tests own verifying that the emitted artifact stays
  synchronized with the contract.

## Non-Goals

- This contract does not add a new readiness artifact file.
- This contract does not change architecture approval gates.
- This contract does not define a new persistence layout for clarification
  answers.