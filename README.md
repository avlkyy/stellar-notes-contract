# Stellar Notes DApp

Blockchain-Based Decentralized Note-Taking System built on Stellar Soroban Smart Contracts.

---

# Overview

Stellar Notes DApp is a decentralized application (DApp) that allows users to create, store, retrieve, and delete notes directly on the Stellar blockchain using Soroban Smart Contracts.

This project demonstrates how blockchain technology can be used for decentralized data storage while maintaining transparency, security, and ownership of user-generated content.

Unlike traditional note-taking applications that rely on centralized databases, Stellar Notes DApp stores data on-chain, ensuring that notes are immutable, verifiable, and censorship-resistant.

---

# Features

## Create Notes

Users can:
- Create notes with title and content
- Store notes permanently on-chain
- Automatically generate unique IDs

## Retrieve Notes

Users can:
- Fetch all notes from blockchain storage
- View stored note data in structured format

## Delete Notes

Users can:
- Delete notes using unique note IDs
- Update blockchain storage instantly

## Blockchain Security

- Decentralized storage
- Immutable records
- Transparent transactions
- Tamper-resistant system

---

# Smart Contract Functions

## `create_note()`

Create a new note.

### Parameters

| Name | Type |
|------|------|
| title | String |
| content | String |

---

## `get_notes()`

Retrieve all stored notes.

### Returns

```json
[
  {
    "id": 1,
    "title": "My Note",
    "content": "Hello Stellar"
  }
]
```

---

## `delete_note()`

Delete a note by ID.

### Parameters

| Name | Type |
|------|------|
| id | u64 |

---

# Smart Contract Structure

## Note Struct

```rust
pub struct Note {
    id: u64,
    title: String,
    content: String,
}
```

---

# Tech Stack

- Rust
- Soroban SDK
- Stellar Blockchain
- WASM Smart Contracts

---

# Contract Information

## Network
Stellar Soroban Testnet

## Contract Address

```text
CBLU4IUASQ4WUMOXBFLZRSBBLILGOH33GS4LUPKFBCCCMJCDQNMF7G2M
```

---

# Project Vision

Our vision is to build a decentralized productivity ecosystem where users fully own and control their digital information without relying on centralized platforms.

The project focuses on:
- Data ownership
- Transparency
- Blockchain security
- Decentralized storage
- User sovereignty

---

# Future Improvements

## Planned Features

- Note encryption
- Wallet authentication
- Categories and tags
- Rich text support
- Search functionality
- Collaborative notes
- IPFS integration
- AI summarization
- Multi-user support
- DAO governance

---

# Installation

## Clone Repository

```bash
git clone https://github.com/your-username/stellar-notes-dapp.git
```

---

# Build Contract

```bash
stellar contract build
```

---

# Run Tests

```bash
cargo test
```

---

# Deploy Contract

```bash
stellar contract deploy \
--wasm target/wasm32-unknown-unknown/release/stellar_notes.wasm \
--source alice \
--network testnet
```

---

# Example Contract Invocation

## Create Note

```bash
stellar contract invoke \
--id CONTRACT_ID \
--source alice \
--network testnet \
-- \
create_note \
--title "Belajar Soroban" \
--content "Smart contract pertama saya"
```

---

## Get Notes

```bash
stellar contract invoke \
--id CONTRACT_ID \
--source alice \
--network testnet \
-- \
get_notes
```

---

## Delete Note

```bash
stellar contract invoke \
--id CONTRACT_ID \
--source alice \
--network testnet \
-- \
delete_note \
--id 1
```

---

# Learning Objectives

This project helps developers learn:
- Soroban smart contract development
- Rust for blockchain
- Stellar blockchain interaction
- Smart contract storage management
- Web3 backend architecture

---

# Author

Developed as a decentralized application project on Stellar Soroban.

---

# License

MIT License