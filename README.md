# 🧾 Pocketbook

**Pocketbook** is an open-source desktop tool for managing [PocketBase](https://pocketbase.io/) instances — built with [Tauri](https://tauri.app/) and powered by PocketBase itself.

Think of it as a GUI-based orchestrator for PocketBase — spin up, manage, and monitor multiple PocketBase projects from a single desktop interface.

---

## ✨ Features

> ✅ = done / stable enough for testing • 🛠️ = planned / in progress

- ✅ Embedded PocketBase (via sidecar) using Tauri
- ✅ Master instance of PocketBase used as controller/state manager
- ✅ Start/stop multiple PocketBase instances
- ✅ Automatic free port detection for new instances
- ✅ Persistent instance metadata management using the master PocketBase
- ✅ GUI interface to:
  - Add new PocketBase instances
  - View active/inactive instances
  - Control instances (start/stop/restart)
- 🛠️ Instance logs and health status view
- 🛠️ Tray integration with start/stop toggle
- 🛠️ Cross-platform support (Windows/Linux/macOS)
- 🛠️ Settings page for defaults like port range, database paths, etc.

---

## 📦 Tech Stack

- [Tauri](https://tauri.app/) – desktop app shell
- [PocketBase](https://pocketbase.io/) – embedded database & backend
- [Rust](https://www.rust-lang.org/) – backend logic
- [React](https://reactjs.org/) – frontend (Tauri-compatible)

---

## 💡 Inspiration

Inspired by [Pockethost](https://pockethost.io), Pocketbook is designed as a lightweight, local-first version for developers who want to manage personal or side-project PocketBase instances **without needing to run a server or use CLI every time**.

Unlike Pockethost, Pocketbook aims to provide:

- A seamless GUI
- Full local control
- Open-source customization

---

## 🚀 Getting Started

### Clone & Build

```bash
git clone https://github.com/BenedictZuze/pocketbook
cd pocketbook
pnpm install
pnpm tauri dev
```

Automatic schema/collection creation is on my TODO. However, here is a manual work-around.

**This has been patched and is no longer required - I have left this here in case anyone wants to manually changes somethings.**

1. On first run you are required to create a super-user account - the credentials should match the values that will be in the .env (check `.env.example`).

2. Next create a collection called `projects` with the following schema:

```json
{
  "name": "projects",
  "type": "base",
  "schema": [
    { "name": "name", "type": "text", "required": true },
    { "name": "port", "type": "number", "required": true },
    {
      "name": "status",
      "type": "select",
      "options": { "values": ["running", "stopped"] },
      "required": true
    },
    { "name": "isHealthy", "type": "bool" },
    { "name": "dataDirectory", "type": "text", "required": true },
    { "name": "pid", "type": "text", "required": true },
    { "name": "createdAt", "type": "date" },
    { "name": "lastStarted", "type": "date" }
  ]
}
```

> 💡 Ensure you have Rust, Node, and Tauri CLI installed.

---

## 📋 Contributing

Pull Requests are welcome! 🙌
If you have ideas, bugs to report, or want to build features, open an issue or submit a PR.

### Contributing Ideas?

Check out the [Discussions](https://github.com/BenedictZuze/pocketbook/discussions) tab or [Issues](https://github.com/BenedictZuze/pocketbook/issues) to pitch ideas or collaborate.

---

## ✅ Roadmap / Feature Checklist

- [x] PocketBase sidecar implementation
- [x] GUI to view/manage instances
- [x] Auto port assignment for instances
- [x] Start/stop/restart instance control
- [x] Store instance metadata in master PocketBase
- [ ] Embed log viewer in the GUI
- [ ] Cross-platform testing
- [ ] Export/import instance configurations
- [ ] Tauri tray menu integration

---

## 📄 License

MIT License © 2025 [Benedict Zuze](https://github.com/BenedictZuze)

---

## 🙏 Acknowledgements

- [PocketBase](https://pocketbase.io)
- [Pockethost](https://pockethost.io) for inspiration
- [Tauri](https://tauri.app) for enabling lightweight native apps
