# Devlog

Devlog is an ambient developer journal daemon that watches coding activity and turns it into useful daily progress notes.

It captures signals from development work such as commits, file changes, and shell activity, then stores them as structured project events. Those events can be summarized into standup updates with an LLM, helping developers remember what they worked on without manually writing logs throughout the day.

Devlog is meant to run quietly in the background and build a timeline of engineering activity across a project, including:

- Git commits and commit metadata
- Changed files and code churn
- Project and branch context
- Local development activity
- LLM-generated daily standup summaries

---

## Demo

[![Devlog Demo](https://img.shields.io/badge/▶%20Watch%20Demo-Google%20Drive-blue?style=for-the-badge&logo=googledrive)](https://drive.google.com/file/d/1BYwaUnQgBFQ6IrJEHyV0k_AihAnewKXY/view?usp=sharing)

---

## Installation

### 1. Clone the repo

```bash
git clone https://github.com/your-username/devlog.git
cd devlog
```

### 2. Build the binaries

```bash
cargo build --release -p devlogd
cargo build --release -p devlog-cli
```

### 3. Install the daemon

```bash
sudo cp target/release/devlogd /usr/local/bin/devlogd
```

### 4. Set up the systemd service

Create `~/.config/systemd/user/devlog.service`:

```bash
mkdir -p ~/.config/systemd/user
```

```ini
[Unit]
Description=Devlog Daemon
After=network.target

[Service]
Type=notify
ExecStart=/usr/local/bin/devlogd
Restart=always
RestartSec=5
NotifyAccess=all

[Install]
WantedBy=default.target
```

Then enable and start it:

```bash
systemctl --user daemon-reload
systemctl --user enable --now devlog.service
```

Verify it's running:

```bash
systemctl --user status devlog.service
```

### 5. Install the CLI

```bash
cd devlog-cli
cargo install --path .
```

---

## Usage

### `devlog setup`

Link a project directory to devlog. Recursively finds all git repos inside the folder and installs a `post-commit` hook so every commit is captured automatically.

```bash
devlog setup --project /path/to/your/projects
```

Or run without the flag to get an interactive prompt:

```bash
devlog setup
```

### `devlog api`

Configure your OpenRouter API key (used for LLM standup summaries).

```bash
devlog api                        # interactive prompt
devlog api --key sk-your-key      # set key directly
devlog api --show                 # print the saved key
devlog api --clear                # remove the saved key
```

The key is saved to `~/.devlog/config` and is used automatically by the daemon.

### `devlog standup`

Generate a daily standup summary from today's captured activity using the LLM.

```bash
devlog standup
```

Output includes:
- **STANDUP** — 3 bullets (Yesterday / Today / Blockers)
- **DEVLOG** — short narrative journal entry
- **KEY INSIGHT** — the hardest problem worked on today
