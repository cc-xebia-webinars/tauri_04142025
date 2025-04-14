# Setup Tauri Project

1. Install [Rust](https://www.rust-lang.org/learn/get-started).

1. Install [Node.js](https://nodejs.org/en/download/).

1. Create a new Tauri project on Linux/macOS:

    ```bash
    npm create tauri-app@latest
    ```

    Select the following options:
    
    ```text
    ✔ Project name · tools-app
    ✔ Identifier · com.tools-app.app
    ✔ Choose which language to use for your frontend · TypeScript / JavaScript - (pnpm, yarn, npm, deno, bun)
    ✔ Choose your package manager · npm
    ✔ Choose your UI template · React - (https://react.dev/)
    ✔ Choose your UI flavor · TypeScript
    ```

1. Install the dependencies:

    ```bash
    cd tools-app
    npm install
    ```

2. Change into the `src` directory:

    ```bash
    cd src
    ```

    Install the `react-router` package:

    ```bash
    npm i react-router
    ```

1. Change into the `src-tauri` directory:

    ```bash
    cd src-tauri
    ```

    Install the `thiserror` crate:

    ```bash
    cargo add thiserror
    ```

1. Download the [Bootstrap](https://github.com/twbs/bootstrap/releases/download/v5.3.5/bootstrap-5.3.5-dist.zip) archive. Extract the archive. Copy `bootstrap.min.css` file into the `src` directory.

1. Run the Tauri application:

    ```bash
    npm run tauri dev
    ```

1. Update the `src/main.tsx` file.

    ```tsx
    import ReactDOM from "react-dom/client";
    import { App } from "./components/App";

    import "./bootstrap.min.css";

    ReactDOM
        .createRoot(document.getElementById("root") as HTMLElement)
        .render(<App />);
    ```

1. Continue programming per the demo.

