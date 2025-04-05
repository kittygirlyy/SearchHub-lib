<p align="center">
  <img src="https://media.tenor.com/OHMxfMcU4eQAAAAi/anime-girl.gif" alt="searchhub-lib" height="200" />
</p>

# ✨ SearchHub-lib

> Some lovely libraries to magically 🔍 **search through FiveM scraps** with SearchHub.

> [!NOTE]  
> This isn't your grandma's search library—this is for elegant princesses who prefer their searches ✨fast✨ and ✨cute✨.

---

## 👑 Author

<p align="left">
  <a href="https://discord.com">
    <img src="https://img.shields.io/badge/Discord-@lauradieuse-5865F2?logo=discord&logoColor=white&style=for-the-badge" alt="Discord Badge"/>
  </a>
  <a href="https://github.com">
    <img src="https://img.shields.io/badge/GitHub-kittygirlyy-181717?logo=github&logoColor=white&style=for-the-badge" alt="GitHub Badge"/>
  </a>
</p>

---

## 🧚‍♀️ What it does

- 🗃️ Loads FiveM scraps magically
- 🔍 Provides enchanting Rust-powered search functionality
- 💖 Returns results faster than your favorite anime waifu

---

## 🚀 Getting Started

### 📦 Add to your Cargo.toml:

```toml
[dependencies]
searchhub-lib = "0.1.0"
```

### 🌸 Quick Rust example

Simply wave your Rust wand ✨:

```rust
use searchhub_lib::search_query;

fn main() {
    let api_key = "your-princess-key✨";
    let query = "cute anime server";

    match search_query(query, api_key) {
        Ok(results) => {
            println!("{:#?}", results);
            for (key, entry) in results.answer {
                println!("🎀 License: {}", entry.license);
                println!("✨ Info: {:?}", entry);
            }
        }
        Err(e) => eprintln!("😿 Error: {:?}", e),
    }
}
```

---

💖 **Happy magical searching!**
