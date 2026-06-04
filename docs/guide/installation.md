# Installation

Canon ships as a single compiled binary named `canon`.

## Debian/Ubuntu (APT)

```bash
curl -fsSL https://apply-the.github.io/packages/apt/gpg.key \
  | sudo gpg --dearmor -o /usr/share/keyrings/apply-the-archive-keyring.gpg

echo "deb [signed-by=/usr/share/keyrings/apply-the-archive-keyring.gpg] https://apply-the.github.io/packages/apt stable main" \
  | sudo tee /etc/apt/sources.list.d/apply-the.list

sudo apt update
sudo apt install canon
```

Later updates:
```bash
sudo apt update
sudo apt upgrade canon
```

## macOS (Homebrew)

```bash
brew tap apply-the/canon && brew install canon
```

Later updates:
```bash
brew update
brew upgrade canon
```

## Windows

Windows support is planned via Scoop and winget. For now, you can build from source.

## Other Options

- **GitHub Release (.deb fallback):** Download `.deb` files directly from the [Releases](https://github.com/apply-the/canon/releases) page.
- **Source install fallback (requires Rust):** Run `cargo install --path .` from the repository root.

## Verify Installation

Verify canon was installed correctly:
```bash
canon --version
```
