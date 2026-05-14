# Expertise Classification Contract

## Purpose

Summarize the stable source-level classification Canon uses to decide whether a
mode may produce a governed expertise input.

## Supported Classification Mapping

- `DomainLanguage` -> `domain-language`
- `DomainModel` -> `domain-model`

## Excluded Modes

The following modes are outside the governed expertise-input surface for this
slice even when they publish useful governed artifacts:
- `Requirements`
- `Discovery`
- `SystemShaping`
- `Architecture`
- `Change`
- `Backlog`
- `Implementation`
- `Refactor`
- `Verification`
- `Review`
- `PrReview`
- `Incident`
- `SecurityAssessment`
- `Migration`
- `SupplyChainAnalysis`
- `SystemAssessment`

## Classification Guarantees

- Supported expertise kinds remain stable for the current contract line.
- Consumers can classify supported expertise inputs without parsing narrative prose.
- A mode without supported expertise classification must not be treated as a governed expertise input for this slice.
