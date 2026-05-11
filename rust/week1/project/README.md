# Bill Manager

A lightweight interactive command-line application for managing personal bills and expenses, written in Rust.

---

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Running the App](#running-the-app)
- [Usage](#usage)
- [Project Structure](#project-structure)
- [Contributing](#contributing)

---

## Overview

Bill Manager is a terminal-based expense tracker that lets you add, view, edit, and remove bills interactively. It was built as a capstone project for Rust Week 1, bringing together concepts like structs, enums, hashmaps, pattern matching, and user input handling.

---

## Features

- Add a bill with a name and amount
- View all existing bills
- Remove a bill by name
- Edit the amount of an existing bill
- Input validation with graceful error handling
- Persistent in-session storage using a `HashMap`

---

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024 or later)
- Cargo (comes bundled with Rust)

Verify your installation:

```bash
rustc --version
cargo --version
```

### Installation

Clone the repository:

```bash
git clone https://github.com/ombima56/stellar-give-impact-bootcamp.git
cd stellar-give-impact-bootcamp/rust/week1/project
```

### Running the App

```bash
cargo run
```

---

## Usage

Once running, you will see an interactive menu:

```
--- Bill Manager ---
1. Add bill
2. View bills
3. Remove bill
4. Edit bill
5. Quit
Enter choice:
```

**Example session:**

```
Enter choice: 1
Enter bill name: Rent
Enter amount: 850

Bill added.

Enter choice: 2
  Rent - $850.00

Enter choice: 4
  Rent - $850.00
Enter bill name to edit: Rent
Enter new amount: 900

Bill updated.

Enter choice: 5
Goodbye!
```

---

## Project Structure

```
project/
├── Cargo.toml
├── README.md
└── src/
    └── main.rs
```

| File | Description |
|------|-------------|
| `src/main.rs` | Application entry point and all logic |
| `Cargo.toml` | Project metadata and dependencies |

---

## Contributing

This project is part of the Stellar Give Impact Bootcamp. Contributions, issues, and feature requests are welcome. Feel free to open a pull request or raise an issue.
