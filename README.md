# ferrit

**ferrit** is a minimal Git-like version control system written in Rust for learning purposes.

Currently implemented:

- `ferrit init` — initializes a new repository by creating a `.git` folder with the required structure
- `ferrit cat-file -p <hash>` — reads and prints the contents of a Git object
- `ferrit hash-object -w <file>` — computes the SHA-1 hash of a file, formats it as a Git blob object, and writes it to `.git/objects/`

---

## Usage

### Build

```bash
cargo build
````

---

### Initialize a repository

```bash
./target/debug/ferrit init
```

This creates a `.git` directory with:

* `objects/`
* `refs/heads/`
* A `HEAD` file pointing to `refs/heads/main`

The folder is now recognized by `git status`.

---

### Create a Git object from a file

```bash
echo -n "hello world" > hello.txt
./target/debug/ferrit hash-object -w hello.txt
```

This prints the SHA-1 hash and stores the compressed object in `.git/objects`.

---

### Inspect a Git object

```bash
./target/debug/ferrit cat-file -p <hash>
```

It prints the original content of the object.
Example output:

```bash
hello world
```

---

## Project Goals

* Learn how Git works under the hood
* Build Git features from scratch using Rust
* Mimic Git CLI behavior step-by-step

---
