# Ark

Simple, fast, and AI-powered version control.

---

## Install

**Linux / Mac**
```bash
curl -fsSL https://raw.githubusercontent.com/sumitt-wayne/Ark/main/install.sh | sh
```

**Via Cargo**
```bash
cargo install ark-vcs
```

**Windows** — Download binary from [Releases](https://github.com/sumitt-wayne/Ark/releases)

---

## Quick Start
```bash
# 1. Start tracking your project
ark start

# 2. Check what changed
ark check

# 3. Save your changes
ark save "your message"

# 4. Push to GitHub
ark push
```

That's it. Four commands and your code is on GitHub.

---

## With AI (Recommended)
```bash
# Setup AI once
ark ai setup

# Then just run one command
ark ai auto
```

One command. AI generates the message, saves, and pushes to GitHub.

---

## All Commands

### Basic

| Command | Description |
|---|---|
| `ark start` | Start tracking your project |
| `ark save "message"` | Save your changes |
| `ark check` | See what changed |
| `ark history` | View save history |
| `ark undo` | Undo last save |
| `ark info` | Show project info |
| `ark scan` | Scan for secrets and API keys |

### GitHub

| Command | Description |
|---|---|
| `ark push` | Push changes to GitHub |
| `ark pull` | Pull changes from GitHub |
| `ark sync` | Pull and push together |
| `ark remote add <url>` | Add GitHub remote |
| `ark remote show` | Show current remote |
| `ark clone <url>` | Clone a repository |

### Branches

| Command | Description |
|---|---|
| `ark branch new <name>` | Create a new branch |
| `ark branch go <name>` | Switch to a branch |
| `ark branch list` | List all branches |
| `ark branch delete <name>` | Delete a branch |
| `ark branch rename <old> <new>` | Rename a branch |
| `ark merge <branch>` | Merge a branch |

### AI Features

| Command | Description |
|---|---|
| `ark ai setup` | Configure Groq API key |
| `ark ai auto` | Auto save and push with AI message |
| `ark ai commit` | Generate smart commit message |
| `ark ai review` | Review your changes |
| `ark ai fix` | Get fix suggestions |
| `ark ai diff` | Explain your changes |
| `ark ai suggest` | Get next step suggestions |
| `ark ai explain` | Explain project history |

### Other

| Command | Description |
|---|---|
| `ark diff` | Show current changes |
| `ark diff <id>` | Show files in a commit |
| `ark tag new <name>` | Create a version tag |
| `ark tag list` | List all tags |
| `ark tag delete <name>` | Delete a tag |
| `ark stash save` | Temporarily save changes |
| `ark stash pop` | Restore stashed changes |
| `ark restore <file>` | Restore a file from last save |

---

## AI Setup

Groq is free — no credit card needed.

1. Go to console.groq.com
2. Sign up with Google
3. Create an API key
4. Run `ark ai setup` and paste your key

Your key is stored encrypted on your machine.

---

## Typical Workflows

### New project
```bash
ark start
ark remote add https://github.com/username/repo.git
ark save "first commit"
ark push
```

### Daily workflow without AI
```bash
ark check
ark save "feat: add login page"
ark push
```

### Daily workflow with AI
```bash
ark ai auto
```

### Working with branches
```bash
ark branch new feature
ark branch go feature
ark save "feat: new feature"
ark branch go main
ark merge feature
ark push
```

---

## Security
```bash
ark scan
```

Detects API keys, passwords, AWS credentials, private keys, and database URLs before you push.

---

## Built With

- Rust
- Groq AI
- Clap
- Serde

---

## Links

- GitHub: https://github.com/sumitt-wayne/Ark
- Crates.io: https://crates.io/crates/ark-vcs
- Releases: https://github.com/sumitt-wayne/Ark/releases

---

## License

MIT — Free to use, modify, and distribute.
