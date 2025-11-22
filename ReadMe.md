# 📦 `Rust Chat UI`

A small Rust library + binary that serves a pre-built web UI (for example, a Vite/TypeScript `dist/` folder).

It can be used:

* **as a library** inside another Rust server (spawned conditionally)
* **as a standalone binary** (run directly from CLI)

This crate is ideal if you want a reusable, embeddable UI server.

---

## ✨ Features

* Serves any static frontend from a `dist/` folder (Vite, React, Svelte, Solid, etc.)
* Simple public API (`start_ui_server(port, path)`)
* Run standalone via:

  ```bash
  cargo run --release --bin chatui
  ```
* Async, non-blocking Axum server
* Works well alongside other Axum/Tokio servers

# ⚡ Install and use globally:

```bash
cargo install --path .
chatui 8080
```
---

# 🚀 Usage as a Standalone Binary

This crate includes a `main.rs`, so you can run the server directly.

### Run default port (3000):

```bash
cargo run --release --bin chatui
```

### Run on custom port:

```bash
cargo run --release --bin chatui -- 8080
```
---

## ✨🖥️ Built-in ChatGPT-like UI Features

- 🌞 **Light Mode**  
- 🌙 **Dark Mode**  
- ⚡ **Fluent Response**  
- 🎞️ **Animations**  
- 🧠 **Thinking Process** *(embedded)*  
- 🗂️ **Chat History Storage**  

- ⚙️ **Settings Panel**:
  > 🔑 OpenAI API Compatible Server URL / Key  
  > 🎛️ Sampling Parameters  
  > 🗄️ Context Cache  
  > 📝 Auto Title Generation

---
![Screenshot of the ChatUI](./screenshot.png)


# 📚 Usage as a Library (Embedded UI Server to your Rust program)

Add to your main server’s `Cargo.toml`:

```toml
[dependencies]
rustchatui = { git = "https://github.com/guoqingbao/rustchatui.git", version="0.1.4" }
```

Then call it conditionally:

```rust
use rustchatui::start_ui_server;
use tokio::task;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Main server...
    let chat_ui_server = task::spawn(async {
        // Port 3000 -> http://localhost:3000
        start_ui_server(3000).await.unwrap();
    });

    // Wait for servers to run
    chat_ui_server.await?;

    Ok(())
}
```

Running multiple tasks along with Chat UI Server

```rust
        let app = ... // use arg.port, e.g, 2999
        let mut tasks = Vec::new();
        // Your other tasks
        tasks.push(tokio::spawn(async move {
            if let Err(e) = axum::serve(listener, app).await {
                eprintln!("Chat API server error: {e:?}");
            }
        }));

        if args.ui_server {
            tasks.push(tokio::spawn(async move {
                // Use this crate with another port
                start_ui_server((args.port + 1) as u16)
                    .await
                    .unwrap();
            }));
        }

        // Run tasks in parallel
        futures::future::try_join_all(tasks)
            .await
            .map_err(candle_core::Error::wrap)?;
```
### Result:

* 🚀 Rust Chat UI server running at http://localhost:3000

---

# 🧠 How It Works

The crate exposes a single function:

```rust
pub async fn start_ui_server(port: u16, dist_path: impl Into<String>) -> Result<()>
```

This:

* Builds a small Axum server
* Serves the `dist/` folder via `ServeDir`
* Listens on `0.0.0.0:<port>`
* Never blocks other async tasks (safe to spawn concurrently)

---

# 📂 Project Structure

```
rustchatui/
  dist/              <-- Your built frontend (npm run build)
  src/
    lib.rs           <-- start_ui_server()
    main.rs          <-- binary entry point
  Cargo.toml
```

**You may copy your other Web frontend build like:**

```bash
# build your typescript project
npm run build
# copy the dist folder to rustchatui to serve
cp -r dist/ rustchatui/dist/
```

---

# 🛠 Building Frontend + Running Server

```
npm run build
cargo run --release --bin chatui
```

---

# 🏁 License

MIT — free to use in personal and commercial projects.

---

If you want, I can also generate:

✅ A full `Cargo.toml`
✅ Example workspace setup
✅ Clap CLI integration
✅ GitHub Actions to auto-copy `dist/` into the crate

Just tell me!
