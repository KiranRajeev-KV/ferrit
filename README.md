# ferrit

**ferrit** is a minimal Git-like version control system written in Rust for learning purposes.

Currently implemented:

- `ferrit init` — initializes a new repository by creating a `.git` folder with the required structure
- `ferrit cat-file -p <hash>` — reads and prints the contents of a Git object

## Usage

### Build

```bash
cargo build
````

### Initialize a repository

```bash
./target/debug/ferrit init
```

This creates a `.git` directory with:

* `objects/`
* `refs/heads/`
* A `HEAD` file pointing to `refs/heads/main`

The folder is now recognized by `git status`.

### Inspect a Git object

If you've created a blob using Git:

```bash
echo "hello ferrit" > test.txt
git hash-object -w test.txt
```

Then use:

```bash
./target/debug/ferrit cat-file -p <hash>
```

It should print `hello ferrit`.

## Project Goals

* Learn how Git works under the hood
* Build Git features from scratch using Rust
* Mimic Git CLI behavior step-by-step
