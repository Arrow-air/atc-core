# Concept of Operations - `atc-core`

<center>

<img src="https://github.com/Arrow-air/tf-github/raw/main/src/templates/doc-banner-services.png" style="height:250px" />

</center>

Attribute | Description
--- | ---
Maintainer | [@Arrow-air/services](https://github.com/orgs/Arrow-air/teams)
Status | Draft
  
## Overview

The `atc-core` library implements the logic, interfaces, and UI components of Arrow's Air Traffic Control (ATC) software.

Two binaries will wrap the `atc-core` library:
1) `atc-operator`: Air traffic control software linked to real flight operations
2) `atc-training`: A training tool with various simulations, levels, and challenges.

The `atc-training` binary may be published before actual UAM operations. Early feedback from "players" regarding UI/UX.

## Related Documents

Document | Description
--- | ---
:construction: Requirements - `atc-core` | Requirements and user stories for this microservice.
[Interface Control Document (ICD) - `atc-core`](./icd.md) | Defines the inputs and outputs of this microservice.
[Software Design Document (SDD) - `atc-core`](./sdd.md) | Specifies the internal activity of this microservice.

## Motivation

Air Traffic Control 

"Airspace" will become more complicated as vertiports. VTOL aircraft operating near street level. Power lines, water towers, antennae. Navigating between sky scrapers.

Need software designed for this environment.

Also needed to train operators and dispatchers for UAM.

## Needs, Goals and Objectives of Envisioned System

Display aircraft
- Type, Size, Direction

Display Vertiports:
- Num Pads, schedules

Display comms channels
- Similar to a traditional ATC interface

## Overview of System and Key Elements



## External Interfaces
See the ICD.

## Proposed Capabilities

## Modes of Operation

TODO

## Operational Scenarios, Use Cases and/or Design Reference Missions

## Nominal & Off-Nominal Conditions

Emergency scenarios

## Physical Environment

The physical location of `atc-core` activities will depend on the binary using the library. This can range from personal laptops to dedicated aviation authority workstations.

## Support Environment

TODO

## Organizational Impacts

Operators and dispatchers will require training in the use of the ATC software.

The `atc-training` binary is purposed toward this.

## Technical Impacts

A well-designed ATC interface dedicated to UAM operations could lead to early UAM operator training. It also visually demonstrates how UAM space can operate safely, which can be persuasive in the face of regulatory challenges.

## Risks and Potential Issues

Operators make split-second decisions based on ATC software visuals. Out-of-date UI elements (positions of aircraft), improper placement of symbols (vertiports), and software lag can critically impact an operator's ability to maintain safe airspace.

## Appendix A: Citations

## Appendix B: Acronyms & Glossary
