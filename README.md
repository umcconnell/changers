![Test](https://github.com/umcconnell/changers/workflows/test/badge.svg)
![Lint](https://github.com/umcconnell/changers/workflows/lint/badge.svg)
![Docs](https://github.com/umcconnell/changers/workflows/docs/badge.svg)

# change\.rs

Make Change. In Rust.

Quickly calculate monetary change for a desired amount, all while benefitting
from Rust's speed and safety.

Change\.rs provides an easy-to-use CLI (command line interface) to calculate
change directly from the command line. Supports ouputing to `stdout` for
results and `stderr` for possible errors.

## Table of Contents

-   [Getting Started](#getting-started)
    -   [Prerequisites](#prerequisites)
    -   [Initial setup](#initial-setup)
-   [Usage](#usage)
-   [Contributing](#contributing)
-   [Versioning](#versioning)
-   [Authors](#authors)
-   [License](#license)

## Getting Started

These instructions will get you a copy of the project up and running on your
local machine.

### Prerequisites

Before you get started, make sure you've got rust and cargo installed on your
machine. If not, installation via rustup is quick and easy. See the rust website
for installation instructions: https://www.rust-lang.org/tools/install

### Initial setup

First, clone the repo with git:

```bash
git clone https://github.com/umcconnell/changers.git
cd changers
```

Then, you can run change\.rs using cargo:

```bash
cargo run <my_amount> examples/coins.txt
```

... or build a binary:

```bash
cargo build --release
```

Check out the [examples](examples/) folder for some example coin files you can
pass.

Happy coding!

## Usage

```
changers [amount] [optional: coin_file]
```

The change\.rs CLI accepts two arguments to calculate change:

-   `amount`: The monetary amount to make change for
-   `coin_file`, optional: An optional coin file (see [examples](examples/)
    folder) containing a list of coins seperated by a newline. If no file is
    passed, coins are read from stdin.

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) and
[CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) for details on our code of conduct, and
the process for submitting pull requests to us.

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available,
see the [tags on this repository](https://github.com/umcconnell/changers/tags).

## Authors

Ulysse McConnell - [umcconnell](https://github.com/umcconnell/)

See also the list of
[contributors](https://github.com/umcconnell/changers/contributors) who
participated in this project.

## License

Change\.rs is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE.md) and [LICENSE-MIT](LICENSE-MIT.md) for
details.

## Acknowledgments

A huge thanks goes to the open and welcoming rust community and their great
documentation effort.

-   https://doc.rust-lang.org/book/
-   https://blog.burntsushi.net/rust-error-handling/
-   https://en.wikipedia.org/wiki/Change-making_problem
