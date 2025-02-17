# Anchor Escrow Program

This README provides an overview of the Anchor Escrow program, a decentralized escrow system built on the Solana blockchain using the Anchor framework.
Escrow in cryptocurrency typically works as follows:
1. The buyer sends funds to the escrow service.
2. The escrow holds the funds securely.
3. The seller delivers the agreed-upon goods or services.
4. Once both parties confirm the transaction is complete, the escrow releases the funds to the seller4.

## Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/whitecometx/Anchor_Escrow
   cd Anchor_Escrow
   ```

2. Install the required dependencies:
   ```bash
   anchor build
   ```

3. Ensure your Solana wallet is set up and funded on the devnet.


## Overview

The Anchor Escrow program facilitates secure token exchanges between two parties (maker and taker) without the need for a trusted intermediary. It supports the following key operations:

1. Make: Initiate an escrow by depositing tokens
2. Take: Complete the escrow by exchanging tokens
3. Refund: Cancel the escrow and return tokens to the maker

## Program Structure

The program consists of the following main components:

1. `state/escrow.rs`: Defines the `Escrow` struct, which stores the escrow data
2. `instructions/make.rs`: Implements the `Make` instruction for creating an escrow
3. `instructions/take.rs`: Implements the `Take` instruction for completing an escrow
4. `instructions/refund.rs`: Implements the `Refund` instruction for cancelling an escrow
5. `lib.rs`: Contains the program's entry points and instruction handlers

## Key Features

- Supports multiple escrows per user using a unique seed
- Uses Program Derived Addresses (PDAs) for secure account derivation
- Implements proper authority checks and constraints
- Utilizes the SPL Token program for token transfers and account management

## Instructions

### Make

Initiates an escrow by depositing tokens from the maker into a vault.

Parameters:
- `seed`: A unique identifier for the escrow
- `deposit`: Amount of tokens to deposit
- `receive`: Amount of tokens expected in return

### Take

Completes the escrow by transferring tokens between the maker and taker.

### Refund

Cancels the escrow and returns deposited tokens to the maker.

## Test

-`anchor_escrow.ts`: Contains tests for the escrow program using the Anchor testing framework.

## Security Considerations

- The program uses PDAs and bump seeds for secure account derivation
- Authority checks ensure only authorized parties can perform specific actions
- Proper account constraints prevent unauthorized access to escrow funds

## Disclaimer

This program is provided as-is and should be thoroughly audited before use in a production environment.
