# **Straw-Hat-Builds**

In the vast ocean of software development, we are all explorers, seeking the ultimate treasure: knowledge. Straw-Hat-Builds is a journey—a path to mastery in blockchain, Rust, and other realms of modern technology. Built with Leptos and leveraging the power of client-side rendering, this platform is a humble step toward empowering developers to build, learn, and grow.

## **Introduction**

Straw-Hat-Builds is more than just a repository; it's an adventure. Drawing inspiration from the unyielding spirit of the Straw Hat Pirates, this project is designed to help you navigate the complex seas of modern development. Whether you're diving into blockchain or exploring the intricacies of Rust, Straw-Hat-Builds is your ship.

## **Features**

- **Leptos Server Side Rendering:** Fast and efficient rendering for modern web applications.
- **Modular Project Guides:** Each project is a map, guiding you through uncharted territories of development.
- **Focus on Blockchain and Rust:** Start your journey in these cutting-edge fields, with plans to expand into other areas.
- **Community-Driven Contributions:** Join a crew of like-minded explorers, contributing to the shared knowledge.

## **Getting Started**

Before you embark on this journey, ensure you have the necessary tools on board.

### **Prerequisites**

- **Rust nightly toolchain:** Make sure Rust's nightly toolchain is installed on your system. If not, you can install it [here](https://www.rust-lang.org/tools/install).
- **cargo-leptos:** Leptos is the core framework used for this project, and it uses cargo-leptos to run Leptos SSR sites. Install it using Cargo:

  ```bash
  cargo install cargo-leptos --locked
  ```

- **wasm32-unknown-unknown:** wasm32-unknown-unknown is a compilation target which allows us to run the application in the browser.

### **Installation**

Clone the repository to your local machine:

```bash

git clone https://github.com/Juan-LukeKlopper/straw-hat-builds.git
```

Navigate to the project directory:

```bash
cd straw-hat-builds
```

### **Running the Project**

To set sail and run the project locally:

```bash
cargo leptos watch
```

Visit `http://localhost:3000` in your browser to explore the project.

### **Building for Production**

When you're ready to deploy your project, you can build it for production:

```bash
cargo leptos build --release
```

This will produce your server binary in `target/server/release` and your site package in `target/site`.

## **Testing Your Project**

To run end-to-end tests, use the following commands:

```bash
cargo leptos end-to-end
```

or, for a release build:

```bash
cargo leptos end-to-end --release
```

Cargo-leptos uses Playwright as the end-to-end test tool. Tests are located in the `end2end/tests` directory.

## **Executing a Server on a Remote Machine Without the Toolchain**

After running a `cargo leptos build --release`, the minimum files needed are:

1. The server binary located in `target/server/release`.
2. The `site` directory and all files within located in `target/site`.

Copy these files to your remote server. The directory structure should be:

```text
straw-hat-builds
└── site/
```

Set the following environment variables (updating for your project as needed):

```bash
LEPTOS_OUTPUT_NAME="straw-hat-builds"
LEPTOS_SITE_ROOT="site"
LEPTOS_SITE_PKG_DIR="pkg"
LEPTOS_SITE_ADDR="127.0.0.1:3000"
LEPTOS_RELOAD_PORT="3001"
```

Finally, run the server binary.

## **Contributing**

We welcome contributions from all corners of the Grand Line. Whether it's a bug fix, a new feature, or a project guide, your input is valued. Please open a pull request, and together, we'll steer this ship forward.

## **License**

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for more details.

## **User-Focused Summary**:
**Straw-Hat Builds** is a platform designed for developers who want to move beyond tutorials and build real products that matter. Inspired by the spirit of **Buildspace**, this platform is more than just a place to learn to code—it's about turning knowledge into action, shipping projects, and releasing them into the world. At Straw-Hat Builds, you embark on a journey where your creations are not just code snippets but functional apps, tools, and websites that others can interact with. Whether you’re learning blockchain, Rust, or other cutting-edge technologies, this is where you grow by building, releasing, and pushing the boundaries of what you can achieve.


## **Investor Pitch**:
**Problem**: Many developers get stuck in a cycle of following tutorials without ever progressing to the next stage: building and shipping real-world applications. This gap between learning and application leaves a massive potential untapped.

**Solution**: **Straw-Hat Builds** solves this by offering a platform that encourages developers to go beyond tutorials and build real, functioning products. Inspired by **Buildspace’s legacy** of motivating creators, we focus on bridging the gap between theory and practice. By guiding developers to create apps, tools, and websites that can be shared and interacted with, we provide them the unique experience of seeing their work in the hands of users.

**Product-Market Fit**: As more industries adopt blockchain and Rust for their performance and security benefits, Straw-Hat Builds offers a valuable pathway for developers looking to specialize in these areas. For companies, the platform creates a pipeline of talent that has already demonstrated their ability to ship production-level code. It’s a space where developer growth aligns with real-world impact, making it ideal for both learners and potential employers.


## **Development Deep Dive**:
**Technical Stack**: **Straw-Hat Builds** is powered by **Leptos**, which handles server-side rendering to create fast, responsive applications. It leverages Rust’s efficiency and security to build robust web applications. Developers can write, ship, and deploy their applications using WebAssembly (wasm32-unknown-unknown), making projects seamless to run in a browser environment.

**Interaction Between Components**:
- **Leptos** enables smooth server-side rendering, crucial for large-scale and performant applications.
- Projects are modular, organized into directories for each guide, allowing users to navigate and learn at their own pace. Each module serves as a self-contained lesson, and is written in markdown and placed into a set folder structure.
- Getting products finished and out in the wild is the current focus areas, but the goal is to grow and refine the lessons to have them focus on all aspects of the code including CI/CD, testing, and how to add onto the project over time.

**Design Choices**: Inspired by **Buildspace’s focus on shipping real products**, Straw-Hat Builds was designed to ensure that developers learn by doing. Instead of keeping their code locked in localhost, the platform encourages users to push their projects into production, reflecting the real-world demands of modern software development. This focus on shipping reflects the deeper purpose of taking responsibility for one's own progress—transforming potential into reality by moving beyond tutorials.

## **Acknowledgements**

Straw-Hat-Builds is inspired by the adventurous spirit of the Straw Hat Pirates and the pursuit of knowledge. A special thanks to the open-source community and to all those who dare to dream and explore the unknown. 

Special shout out to kent-3, his work into using keplr with leptos really helped push me into making this project using Rust! You are the GOAT sir!
