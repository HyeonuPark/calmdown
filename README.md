Calmdown
=========

> Calm down and check the code again

This tiny utility does the single job well:
it fails if the files under the directory modified too recently.

Initial motivation is to use it within a git hook, to prevent to push hastly.
But who knows its limitation?

# Usage

To fail within 10 minutes after modification:
> $ calmdown -m 10

Apply to multiple directories:
> $ calmdown -h 1 ./foo ./bar

Or just check the last modification time without failing:
> $ calmdown

# Installation

Currently you need the Rust language toolchain to install calmdown.
If you want to install calmdown without it, please file an issue on the github.

> $ cargo install calmdown
