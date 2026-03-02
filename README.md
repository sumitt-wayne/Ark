# Ark

A simple, fast, and AI-powered version control tool built with Rust.

Ark makes version control easy for everyone — from beginners to experienced developers.

---

## Why Ark?

Git is powerful but complex. Ark simplifies the workflow without losing any power.
```
# Git way
git add .
git status
git commit -m "think of a message..."
git push origin main

# Ark way
ark ai auto
```

---

## Installation

### Requirements
- Rust (1.70 or higher)
- Git

### Install from source
```bash
git clone https://github.com/sumitt-wayne/Ark.git
cd Ark
cargo install --path .
```

Verify installation:
```bash
ark --version
```

---

## Quick Start
```bash
# Start tracking your project
ark start

# Check what changed
ark check

# Save your changes
ark save "your message"

# View history
ark history

# Push to GitHub
ark sync
```

---

## All Commands

### Basic

| Command | Description |
|---|---|
| `ark start` | Initialize a new Ark repository |
| `ark save "message"` | Save your changes with a message |
| `ark check` | See what files changed |
| `ark history` | View commit history |
| `ark undo` | Undo last save |
| `ark info` | Show project info |
| `ark sync` | Push and pull from remote |
| `ark scan` | Scan for secrets and API keys |

### Branches

| Command | Description |
|---|---|
| `ark branch new <name>` | Create a new branch |
| `ark branch go <name>` | Switch to a branch |
| `ark branch list` | List all branches |
| `ark branch delete <name>` | Delete a branch |
| `ark branch rename <old> <new>` | Rename a branch |
| `ark merge <branch>` | Merge a branch into current |

### AI Features

| Command | Description |
|---|---|
| `ark ai setup` | Configure your Groq API key |
| `ark ai commit` | Generate a smart commit message |
| `ark ai review` | Get a code review of your changes |
| `ark ai fix` | Get fix suggestions for your code |
| `ark ai auto` | Auto generate message, save and push |
| `ark ai explain` | Explain recent project history |
| `ark ai diff` | Explain your current changes |
| `ark ai suggest` | Get suggestions for next steps |

### Other

| Command | Description |
|---|---|
| `ark diff` | Show current changes |
| `ark diff <commit-id>` | Show files in a specific commit |
| `ark remote add <url>` | Add a remote repository |
| `ark remote show` | Show current remote |
| `ark clone <url>` | Clone a repository |
| `ark tag new <name> "message"` | Create a version tag |
| `ark tag list` | List all tags |
| `ark tag delete <name>` | Delete a tag |
| `ark stash save "message"` | Temporarily save changes |
| `ark stash list` | List all stashes |
| `ark stash pop` | Restore last stashed changes |
| `ark stash drop` | Delete last stash |
| `ark restore <file>` | Restore a file from last commit |

---

## AI Setup

Ark uses Groq for AI features. Groq is completely free — no credit card needed.

### Steps

1. Go to console.groq.com
2. Sign up with Google (takes 2 minutes)
3. Go to API Keys section
4. Click "Create API Key"
5. Copy your key
6. Run:
```bash
ark ai setup
```

Paste your key when prompted. Done.

Your API key is stored encrypted on your machine. It never leaves your computer.

---

## Typical Workflow

### Without AI
```bash
ark start
ark remote add https://github.com/username/repo.git
ark check
ark save "feat: add login page"
ark sync
```

### With AI (Recommended)
```bash
ark start
ark remote add https://github.com/username/repo.git
ark ai setup

# make your changes...

ark ai auto
# message generated, saved, and pushed in one command
```

### With Branches
```bash
ark branch new feature-login
ark branch go feature-login

# make your changes...

ark ai commit
ark branch go main
ark merge feature-login
ark sync
```

---

## Security

Ark has built-in secret scanning. Before pushing sensitive projects, run:
```bash
ark scan
```

Ark will detect:
- API keys
- Passwords
- AWS credentials
- Private keys
- Database URLs

---

## Built With

- Rust
- Groq AI (llama-3.3-70b-versatile)
- Clap
- Serde

---

## License

MIT License. Free to use, modify, and distribute.

---

## Contributing

Contributions are welcome. Open an issue or submit a pull request.

GitHub: https://github.com/sumitt-wayne/Ark
