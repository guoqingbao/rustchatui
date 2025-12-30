# 📦 `Rust Chat UI`

A small Rust library + binary that serves a pre-built web UI (for example, a Vite/TypeScript `dist/` folder).

It can be used:

* **as a library** inside another Rust server (spawned conditionally)
* **as a standalone binary** (run directly from CLI)

This crate is ideal if you want a reusable, embeddable UI server.

💡This crate is a Rust wrapper for the **release build** of `chatclient` (co-developed with Gemini): [https://github.com/guoqingbao/chatclient](https://github.com/guoqingbao/chatclient)

---

## ✨ Features

* Serves any static frontend from a `dist/` folder (Vite, React, Svelte, Solid, etc.)
* Simple public API (`start_ui_server(port, path)`)
* Run standalone via:

  ```bash
  cargo run --release --bin chatui --ui-port 8080 --api-port 8000
  ```
* Async, non-blocking Axum server
* Works well alongside other Axum/Tokio servers

# ⚡ Install and use globally:

```bash
cargo install --path .
# Use local API server
chatui --ui-port 8080 --api-port 8000
# Use remote API server
chatui --ui-port 8080 --server-url http://api.openai.com/v1 --api-key xxxxx
```
---

# 🚀 Usage as a Standalone Binary

This crate includes a `main.rs`, so you can run the server directly.

### Run on port (8080) and communicate with local (port 8000) API server:

```bash
cargo run --release --bin chatui -- --ui-port 8080 --api-port 8000
```

### Run on port (8080) and communicate with remote API server:

```bash
cargo run --release --bin chatui -- --ui-port 8080 --server-url http://api.openai.com/v1 --api-key xxxxx
```

---

## ✨🖥️ Built-in ChatGPT-like UI Features

- 🌞 **Light Mode**  
- 🌙 **Dark Mode**  
- ⚡ **Fluent Response**  
- 🎞️ **Animations**  
- 🧠 **Thinking Process** *(embedded)*  
- 🗂️ **Chat History Storage**  
- 📈 **Token Usage Indicator**

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
rustchatui = { git = "https://github.com/guoqingbao/rustchatui.git", version="0.2.8" }
```

Then call it conditionally:

```rust
use rustchatui::start_ui_server;
use tokio::task;

// Running multiple tasks along with Chat UI Server

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
                start_ui_server(port as u16, (args.port + 1) as u16, None, None)
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
async fn start_ui_server(
    ui_port: u16, // Port for this UI web server
    api_port: Option<u16>, // Port for Local API server
    server_url: Option<String>, // Remote API server url: http://api.openai.com/v1
    api_key: Option<String>, // api key for local or remote API server
) -> Result<()> 
```

This:

* Builds a small Axum server
* Serves the `dist/` folder via `ServeDir`
* Listens on `0.0.0.0:<port>`
* Call local server if `api_port` configured or call remote api server if `server_url` provided
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

You may copy your other Web frontend build:

```bash
# build your typescript project
npm run build
# copy the dist folder to rustchatui to serve
cp -r dist/ rustchatui/dist/
```

---

# 🏁 License

MIT — free to use in personal and commercial projects.
