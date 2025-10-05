# 🖥️ [Tauri Desktop Applications | Rust Workshop](https://rust.ipworkshop.ro/docs/tauri/)

>  Tauri + Vanilla TS

## 📥 Instalare dependinte (pe Linux)

```sh
sudo apt update
sudo apt upgrade -y

sudo apt install -y npm nodejs

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


Din directorul `tauri-app`, ruleaza:

```sh
deno task tauri dev        # Porneste GUI-ul
```
