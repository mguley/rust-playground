### Rust Playground

A hands-on learning environment for Rust programming patterns, memory safety concepts, and implementation strategies.
This repository contains practical scenarios that demonstrate Rust concepts through guided exercises.

### Overview

This playground is designed to help developers learn Rust by doing.
Each scenario focuses on a specific pattern or technique used in production Rust environments.
The scenarios are self-contained and include step-by-step instructions, code samples, and explanations.

We follow a progressive learning approach - starting from foundational concepts and incrementally building toward more advanced topics.

### Prerequisites

Before starting, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install) (version 1.85+ recommended, tested with 1.92)
- A code editor of your choice
- Basic understanding of programming concepts
- Terminal/command-line access

### Getting Started

```bash
# Clone this repository
git clone https://github.com/mguley/rust-playground.git
cd rust-playground

# Verify your Rust installation
rustc --version
cargo --version

# View available scenarios
ls -la
```

### Available Scenarios

#### [Scenario 1: Common Collections in Rust](./scenario-01-common-collections-in-rust/)

Master Rust's standard collection types and understand when to use each one.
This scenario covers all eight primary collection types in Rust's standard library, demonstrating their
strengths, performance characteristics, and practical use cases.

**Key Topics:**
- Sequences: `Vec`, `VecDeque`, `LinkedList`
- Maps: `HashMap`, `BTreeMap`
- Sets: `HashSet`, `BTreeSet`
- Priority queues: `BinaryHeap`
- Performance characteristics and when to use each collection
- Iterators, capacity management, and the Entry API
