# 🖥️ [Tauri Desktop Applications | Rust Workshop](https://rust.ipworkshop.ro/docs/tauri/)

>  Tauri + Vanilla TS

## 📥 Instalare dependinte (pe Linux)

```sh
sudo apt update
sudo apt upgrade -y

sudo apt install -y npm nodejs

# Upgrade npm
npm install -g npm@latest

# Upgrade Node.js (recommended)
nvm install 22
nvm use 22


# Install Deno:
curl -fsSL https://deno.land/x/install/install.sh | sh


# Install Rust (skip if you already have):
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

sudo apt install -y \
  libwebkit2gtk-4.1-dev \
  libjavascriptcoregtk-4.1-dev \
  libsoup-3.0-dev \
  pkg-config \
  build-essential \
  libssl-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev

cargo install tauri-cli
```


## Dependinte suplimentare pentru `deno`

Din directorul `tauri-app`, ruleaza:

```sh
deno task tauri add dialog
```

## Deploy pe un alt sistem Linux

```sh
deno task tauri build      # compileaza
```

## GUI

```sh
deno task tauri dev        # Porneste GUI-ul
```
