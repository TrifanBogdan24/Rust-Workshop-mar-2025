
# [Tauri Desktop Applications | Rust Workshop](https://rust.ipworkshop.ro/docs/tauri)


## Instalare NPM

Install **Node.js** with **npm** in Linux:
```sh
sudo apt install npm nodejs
```


Instalare biblioteci necesare:
  

```sh
sudo apt update
sudo apt install

sudo apt install -y \
  libjavascriptcoregtk-4.1-dev \
  libwebkit2gtk-4.1-dev \
  libwebkit2gtk-4.0-dev \
  libjavascriptcoregtk-4.0-dev \
  libsoup-3.0-dev \
  libgtk-3-dev \
  libglib2.0-dev \
  libgdk-pixbuf2.0-dev \
  build-essential \
  pkg-config \
  libssl-dev \
  curl
```



## Troubleshoot instalare NPM

In cazul in care versiunea pe care `npm` o instalareaza este veche (mai veche decat Node 14+),
ruleaza:


```sh
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash
source ~/.bashrc    # sau ~/.zshrc, dupa caz
nvm install 18
nvm use 18
```



## Creare template Tauri


Run the following command to create a new Tauri project with Vite and TypeScript:
```sh
deno run -A npm:create-tauri-app@latest
```

## Tauri + Vanilla TS

This template should help get you started developing with Tauri in vanilla HTML, CSS and Typescript.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)



```sh
  cd Rust-Workshop-mar-2025
  deno install
  deno task tauri android init

For Desktop development, run:
  deno task tauri dev

For Android development, run:
  deno task tauri android dev

```


## Pornirea aplicatiei Tauri


```sh
npm install
npm run tauri dev
```