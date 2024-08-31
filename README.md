<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# Leptos Axum Starter Template

This is a template for use with the [Leptos](https://github.com/leptos-rs/leptos) web framework and the [cargo-leptos](https://github.com/akesson/cargo-leptos) tool using [Axum](https://github.com/tokio-rs/axum).

## Creating your template repo

If you don't have `cargo-leptos` installed you can install it with

```bash
cargo install cargo-leptos --locked
```

Then run
```bash
cargo leptos new --git leptos-rs/start-axum
```

to generate a new project template.

```bash
cd straw-hat-builds
```

to go to your newly created project.  
Feel free to explore the project structure, but the best place to start with your application code is in `src/app.rs`.  
Addtionally, Cargo.toml may need updating as new versions of the dependencies are released, especially if things are not working after a `cargo update`.

## Running your project

```bash
cargo leptos watch
```

## Installing Additional Tools

By default, `cargo-leptos` uses `nightly` Rust, `cargo-generate`, and `sass`. If you run into any trouble, you may need to install one or more of these tools.

1. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
2. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
3. `cargo install cargo-generate` - install `cargo-generate` binary (should be installed automatically in future)
4. `npm install -g sass` - install `dart-sass` (should be optional in future

## Compiling for Release
```bash
cargo leptos build --release
```

Will generate your server binary in target/server/release and your site package in target/site

## Testing Your Project
```bash
cargo leptos end-to-end
```

```bash
cargo leptos end-to-end --release
```

Cargo-leptos uses Playwright as the end-to-end test tool.  
Tests are located in end2end/tests directory.

## Executing a Server on a Remote Machine Without the Toolchain
After running a `cargo leptos build --release` the minimum files needed are:

1. The server binary located in `target/server/release`
2. The `site` directory and all files within located in `target/site`

Copy these files to your remote server. The directory structure should be:
```text
straw-hat-builds
site/
```
Set the following environment variables (updating for your project as needed):
```text
LEPTOS_OUTPUT_NAME="straw-hat-builds"
LEPTOS_SITE_ROOT="site"
LEPTOS_SITE_PKG_DIR="pkg"
LEPTOS_SITE_ADDR="127.0.0.1:3000"
LEPTOS_RELOAD_PORT="3001"
```
Finally, run the server binary.

## Licensing

This template itself is released under the Unlicense. You should replace the LICENSE for your own application with an appropriate license if you plan to release it publicly.


# **Straw-Hat-Builds**

In the vast ocean of software development, we are all explorers, seeking the ultimate treasure: knowledge. Straw-Hat-Builds is a journey—a path to mastery in blockchain, Rust, and other realms of modern technology. Built with Leptos and leveraging the power of client-side rendering, this platform is a humble step toward empowering developers to build, learn, and grow.

## **Introduction**

Straw-Hat-Builds is more than just a repository; it's an adventure. Drawing inspiration from the unyielding spirit of the Straw Hat Pirates, this project is designed to help you navigate the complex seas of modern development. Whether you're diving into blockchain or exploring the intricacies of Rust, Straw-Hat-Builds is your ship.

## **Features**
- **Leptos Client-Side Rendering:** Fast and efficient rendering for modern web applications.
- **Modular Project Guides:** Each project is a map, guiding you through uncharted territories of development.
- **Focus on Blockchain and Rust:** Start your journey in these cutting-edge fields, with plans to expand into other areas.
- **Community-Driven Contributions:** Join a crew of like-minded explorers, contributing to the shared knowledge.

## **Getting Started**

Before you embark on this journey, ensure you have the necessary tools on board.

### **Prerequisites**
- **Rust:** Make sure Rust is installed on your system. If not, you can install it [here](https://www.rust-lang.org/tools/install).
- **Trunk:** Leptos is the core framework used for this project, and it uses trunk to run Leptos CSR sites. Install it using Cargo:
  ```bash
  cargo install trunk
  ```
- **wasm32-unknown-unknown:** wasm32-unknown-unknown is a compilation target which allows us to run the application in the browser

### **Installation**

Clone the repository to your local machine:
```bash
git clone https://github.com/Juan-LukeKlopper/straw-hat-builds.git
```
Navigate to the project directory:
```bash
cd straw-hat-builds
```
Install the necessary dependencies:
```bash
cargo install
```

### **Running the Project**

To set sail and run the project locally:
```bash
trunk serve
```
Visit `http://localhost:8080` in your browser to explore the project.

### **Building for Production**

When you're ready to deploy your project, you can build it for production:
```bash
trunk build --release
```
The production-ready files will be located in the `dist` directory.

## **Contributing**

We welcome contributions from all corners of the Grand Line. Whether it's a bug fix, a new feature, or a project guide, your input is valued. Please open a pull request, and together, we'll steer this ship forward.

## **License**

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for more details.

## **Acknowledgements**

Straw-Hat-Builds is inspired by the adventurous spirit of the Straw Hat Pirates and the pursuit of knowledge. A special thanks to the open-source community and to all those who dare to dream and explore the unknown.
