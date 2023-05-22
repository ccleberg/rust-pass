# rpass

A simple command-line password manager, written in Rust + SQLite. This tool
allows you to manage accounts and generate random passwords containing ASCII
letters, numbers, and punctuation or XKCD-like passphrases.

Data is encrypted prior to being saved within the SQLite database using the
[fernet](https://docs.rs/fernet/) crate. Encryption and decryption require the
use of a randomly-generated key saved as `vault.key` but the key-file is saved
in plaintext, which means that an attacker that can access the key-file can also
decrypt the database. Further development may allow password-protection of the
key-file; please open an issue or pull request if you want this feature!

---

**NOTE:** This crate is not ready for use in production yet. There are many
items still left to implement prior to a production-ready release - see the
[TODO](#todo) section for more details.

---

## Table of Contents

-   [Usage](#usage)
    -   [Arguments](#arguments)
    -   [Examples](#examples)
-   [Contributing](#contributing)
    -   [TODO](#todo)
    -   [Development](#development)

## Usage

[(Back to top)](#table-of-contents)

All arguments can be passed to the app with the following template:

```bash
rpass ARGUMENT [VALUES]
```

### Arguments

#### Summary

<table>
  <thead>
    <tr>
      <td><b>Argument (Short)</b></td>
      <td><b>Argument (Long)</b></td>
      <td><b>Explanation</b></td>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>-h</td>
      <td>--help</td>
      <td>Print the welcome message</td>
    </tr>
    <tr>
      <td>-n</td>
      <td>--new</td>
      <td>Create a new account</td>
    </tr>
    <tr>
      <td>-l</td>
      <td>--list</td>
      <td>List all saved accounts</td>
    </tr>
    <tr>
      <td>-e</td>
      <td>--edit [UUID] [FIELD_NAME]</td>
      <td>Edit a saved account</td>
    </tr>
    <tr>
      <td>-d</td>
      <td>--delete [UUID]</td>
      <td>Delete a saved account</td>
    </tr>
    <tr>
      <td>-p</td>
      <td>--purge</td>
      <td>Purge all accounts and delete the vault</td>
    </tr>
  </tbody>
</table>

## Contributing

[(Back to top)](#table-of-contents)

Any and all contributions are welcome. Feel free to fork the project, add
features, and submit a pull request.

### TODO:

-   [x] Create an account in memory (as a `struct`)
-   [x] Allow random password generation
-   [x] Allow random passphrase generation
-   [x] Create an empty database or file, if not created yet
-   [x] Save new accounts to database or file
-   [x] Pretty-print all saved accounts
-   [ ] Allow editing of a saved account
-   [ ] Allow deletion of a saved account
-   [ ] Allow purging the database
-   [x] Allow user encryption of database or file
-   [x] Allow user-created keys to automatically encrypt/decrypt the database or
        file
-   [ ] Require password to encrypt/decrypt/hash/salt/etc. the key file.
-   [ ] Create test suite
-   [ ] Publish to crates.io when the package is in a minimally-usable state
-   [ ] Restructure and format code according to best practices (dead code,
        unused imports, etc.)

## Development

### Setup

```bash
git clone REPO_URL
```

```bash
cd REPO
```

#### Local Testing

If you've made changes to the code and would like to test them, use the
following commands.

```bash
cargo build --release
```

```bash
./target/release/REPO --help
```

#### Building & Publishing

If you are ready to push your changes to crates.io, use the commands below. For
beginners, note that you cannot publish changes to a crate you don't own (i.e.
you must be added as a contributor on crates.io).

```bash
cargo build --release
```

```bash
cargo login [API_TOKEN]
```

```bash
cargo publish --dry-run
```

```bash
cargo publish
```
