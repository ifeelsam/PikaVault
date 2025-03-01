# PikaVault 
#### Pika Vault is a RWA marketplace for collectibles such as TCG Cards, enabling users to mint, list, purchase, and manage NFTs on befalf of Real collectibles. 


## Features

- **Mint and List NFTs**: Create NFTs and list them for sale.
- **Purchase with Escrow**: Secure funds with escrow until the card reaches your doorstep.
- **Delist NFTs**: Remove listed NFTs from the marketplace.
- **User Accounts**: Track user activity (NFTs bought, sold, and listed).
- **Marketplace Management**: Administer marketplace fees and treasury.

---

<!-- ## Project Structure

```
src
├── constants.rs          # Constants used across the program
├── error.rs              # Custom error definitions
├── instructions          # Core logic for marketplace operations
│   ├── initialize.rs     # Initialize the marketplace
│   ├── register_user.rs  # Register user accounts
│   ├── list.rs           # Mint and list NFTs
│   ├── delist.rs         # Delist NFTs
│   ├── purchase.rs       # Purchase NFTs with escrow
│   └── release.rs        # Release escrow funds
├── state                 # Data structures for accounts
│   ├── marketplace.rs    # Marketplace metadata
│   ├── user_account.rs   # User account data
│   ├── lisitngs.rs       # NFT listing details
│   └── escrow.rs         # Escrow account for transactions
└── lib.rs                # Entry point of the program
```

--- -->

## Architecture Diagram

<img src="https://cdn.edaquest.com/Arch%20Diagram%20(2).png" />



## Documents Included

1.  [**LOI (Letter of Intent)**](https://cdn.edaquest.com/Arch%20Diagram%20(17).pdf): Outlines the project's objectives and goals.
2.  [**Architecture Diagram**](https://cdn.edaquest.com/Sam%20-%20Capstone%20Letter%20Of%20Intent%20.pdf): Visual representation of the system's structure.


## How to Use

### 1. Initialize Marketplace:

```rust
initialize_marketplace(ctx, fee);
```


### 2. Register User:

```rust
register_user(ctx);
```


### 3. Mint and List NFT:

```rust
mint_and_list(ctx, name, symbol, listing_price, card_metadata, image_url);
```


### 4. Purchase NFT:

```rust
purchase(ctx);
```


### 5. Delist NFT:

```rust
delist(ctx);
```


### 6. Release Escrow:

```rust
release_escrow(ctx);
```

---
