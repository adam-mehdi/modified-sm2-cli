# Modified SM-2 Scheduler

**See the `master` branch for the code**

A command-line application implementing a modified version of the SM-2 algorithm for spaced repetition learning. The SM-2 algorithm helps to determine the optimal intervals for reviewing flashcards or other learning materials to maximize retention. This modified version provides saner defaults for the learning phase.
## Features

    Calculates intervals, repetitions, and ease factors based on user-input quality scores
    Allows users to simulate a series of responses and observe the resulting intervals
    Written in Rust for performance and safety

## Getting Started
### Prerequisites

    Rust: To install Rust, follow the instructions on the official Rust website: https://www.rust-lang.org/tools/install

### Installation

    Clone the repository:


```
git clone https://github.com/your_username/your_repository_name.git
```

Navigate to the project directory:

```
cd your_repository_name
```

Build the project:


```
cargo build --release
```

The compiled binary will be located in the target/release folder. You can move it to a directory in your $PATH for easy access.

### Usage

To run the application, use the following command in your terminal:

```
cargo run -- -q [QUALITY_SCORES]
```

Replace [QUALITY_SCORES] with a series of space-separated integers representing quality scores (between 0 and 5) for each repetition.

Example:

```
cargo run -- -q 4 4 4 4 3 2 1 2 3 4 5
```

This will output the intervals after each response:


```
Interval after response 1: 3
Interval after response 2: 7
Interval after response 3: 18
...
```
