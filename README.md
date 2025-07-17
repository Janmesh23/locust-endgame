

# 🛰️ Locust

> *“Swarm in silence. Track your trail.”*

**Locust** is a Rust-powered CLI app that logs your approximate location every 30 minutes and visualizes it as a heatmap — all offline, all yours.

It’s like a private version of Google Location History: no surveillance, no data selling, no cloud. Just a quiet trail of everywhere you’ve been, stored locally on your machine.

---

## ✨ Features

* 📍 Tracks your location via IP geolocation API
* 🔁 Logs every 30 minutes (or custom interval)
* 🌍 Generates an interactive heatmap in your browser
* 🧠 Stores logs in a simple JSONL file (`locations.jsonl`)
* 🔒 100% local and private — nothing leaves your machine
* 🧪 Simple, hackable, and extensible

---

## 🚀 Quick Demo

```bash
$ locust start
Logging location every 30 minutes...

$ locust map
✅ Heatmap generated at ./map/index.html
🌐 Opening in browser...
```

---

## 📦 Installation

```bash
git clone https://github.com/yourusername/locust
cd locust
cargo build --release
./target/release/locust --help
```

---

## 🛠️ Commands

| Command  | Description                            |
| -------- | -------------------------------------- |
| `start`  | Starts logging your location           |
| `map`    | Generates a heatmap and opens it       |
| `list`   | Lists recent location entries          |
| `config` | Set custom interval, log path, API URL |

---

## 🔧 Configuration

You can create a `config.json` in the project root:

```json
{
  "interval": 1800,
  "log_path": "locations.jsonl",
  "api_url": "https://ip-api.com/json"
}
```

---

## 📍 How Location Works

Locust uses a geolocation API (like `ip-api.com` or `ipapi.co`) to get your latitude and longitude from your IP. It’s not GPS-level accurate, but it’s great for tracking general movement over time.

---

## 🔐 Privacy

All data is stored locally. Locust does **not** send your logs to any server. You're in control of your data — and you can delete it anytime.

---

## 🧠 Why "Locust"?

Like a locust swarm, it moves silently and leaves behind a trace — a pattern of movement. This app does the same, logging where you go and mapping your digital presence.

---

## 📚 Roadmap Ideas

* [ ] Add CSV export
* [ ] Encrypt log file (with a master password)
* [ ] Add time range filters for maps
* [ ] Real-time live tracking view
* [ ] GUI wrapper (Tauri or egui)

---

## 🤖 Built With

* [Rust](https://www.rust-lang.org/)
* [reqwest](https://docs.rs/reqwest)
* [serde](https://serde.rs/)
* [chrono](https://docs.rs/chrono)
* [clap](https://docs.rs/clap)
* [Leaflet.js](https://leafletjs.com/) (for map rendering)

---

## 🧑‍💻 Author

Made by \[your-name]
Rust enthusiast, builder of weird & useful tools 🦀
Feel free to fork, hack, and suggest features!

---


