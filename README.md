# npmrus

A Rust CLI tool to scaffold and manage a basic **Express.js** server project.

## Features

- Create a new Express project
- Add new routes to the server
- Build the Express project with necessary boilerplate
- Fast and minimal setup

---

## Installation

Clone the repository and build the project:

```bash
git clone https://github.com/your-username/npmrus.git
cd npmrus
cargo build --release
```

This will generate the executable in target/release/npmrus.
You can also run it with:
```bash
cargo run -- [OPTIONS]
```

## Run the tool with:
```bash
npmrus [OPTIONS]
```

| Option                       | Description                          |
| ---------------------------- | ------------------------------------ |
| `--createproject <NAME>`     | Create a new Express.js project      |
| `--createroute <ROUTE_NAME>` | Add a new route to your project      |
| `--build <PROJECT_NAME>`     | Set up the Express project structure |
| `-h, --help`                 | Show help                            |
| `-V, --version`              | Show version                         |


## Example
Create a new project
```bash
npmrus --createproject myserver
```

Add a new route to the project
```bash
npmrus --createroute myroute
```

Build the project
```bash
npmrus --build myserver
```

---
