# Rust Blockchain

A proof-of-stake blockchain implementation in Rust that demonstrates core blockchain concepts including consensus, block creation, and validator participation.

## Features

- Proof of Stake (PoS) consensus mechanism
- Multi-validator support
- SQLite-based block storage
- Weighted random validator selection
- Genesis block generation
- Block validation
- Candidate pool management

## Prerequisites

- Rust (latest stable version)
- SQLite
- pkg-config (for Unix-like systems)

## Environment Setup

Create a `.env` file in the project root with the following variables:

```env
DEFAULT_BLOCK_TIME=60
DEFAULT_BLOCK_DB_PATH="./blocks.db"
DEFAULT_CANDIDATE_DB_PATH="./candidates.db"
```

## Building and Running

```bash
# Build the project
cargo build

# Run the blockchain node
cargo run
```

## Project Structure

```
src/
├── main.rs              # Application entry point
├── blockchain/          # Core blockchain implementation
│   ├── block.rs        # Block structure and methods
│   └── blockchain.rs   # Blockchain state management
├── staking/            # Proof of Stake implementation
│   └── validator.rs    # Validator logic and consensus
├── mempool/            # Transaction and block pool management
└── util.rs             # Utility functions and helpers
```

## Core Components

### Block Structure
Each block contains:
- Index
- Timestamp
- Hash
- Previous block hash
- Validator information
- Transaction data (BPM in current implementation)

### Consensus Mechanism
The blockchain uses a Proof of Stake (PoS) consensus mechanism where:
1. Validators propose blocks based on their stake
2. Each validator has an equal chance to be selected (current implementation)
3. Block selection uses a weighted random algorithm
4. The selected block is added to the chain

### Storage
The project uses SQLite for:
- Block storage (confirmed blocks)
- Candidate storage (proposed blocks)

## Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_block_store
cargo test test_candidate_store
```

## Known Limitations

- Fixed validator stakes (1000 each)
- No transaction pool implementation
- Simplified block structure
- Basic consensus mechanism

## Future Improvements

1. Advanced Features:
   - Transaction pool implementation
   - Dynamic validator stakes
   - Slashing conditions
   - Block rewards

2. Security Enhancements:
   - Cryptographic signatures
   - Malicious behavior detection
   - Fork resolution

3. Performance Optimizations:
   - Parallel block validation
   - Enhanced database queries
   - Memory pool optimization

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.