# 🧠 NFT Holder Recognizer Discord Bot (Sui Blockchain)

This is a Discord bot that interacts with the [Sui blockchain](https://sui.io/) to verify wallet ownership and assign roles based on NFT ownership. Users will verify their wallet, prove they own a specific NFT, and be granted the role of **"Dao Member"** in your Discord server.

---

## 📦 Features

- 🔗 Integrates with the Sui blockchain's fullnode to check wallet ownership using `sui_sdk`
- 🧾 Verifies if a user holds a specific NFT based on contract address
- 🤖 Automatically assigns the **"Dao Member"** role to users who own the required NFT
- 🛠️ Works within a Discord channel using a simple bot interface

---

## 🔧 Dependencies

In your `Cargo.toml`, include the following dependencies:

```toml
[dependencies]
sui_sdk = { git = "https://github.com/mystenlabs/sui", package = "sui-sdk" }
tokio = { version = "1.2", features = ["full"] }
anyhow = "1.0"
serenity = "0.11"  # For interacting with Discord API
serde = { version = "1.0", features = ["derive"] }  # For serializing and deserializing data
serde_json = "1.0"  # For handling JSON data
