[![dependency status](https://deps.rs/repo/github/saizo80/Secure-Remove/status.svg)](https://deps.rs/repo/github/saizo80/Secure-Remove)

# Secure Remove CLI

A terminal program for securely removing files with sensitive data, written in Rust.

While using the Eraser software for windows, I needed to be able to securely erase sensitive files
from the terminal, and thus be able to call it in scripts. So I decided to write
my own application to serve that purpose. I originally wrote the script in C#, but decided to move
to Rust for the speed benefits.

The project is uploaded to Crates.io and can be seen [here](https://crates.io/crates/secure_remove).

## Installation

On GitHub select your architecture from the latest release and unzip the archive.

You will have the file named `srm` (`srm.exe` on Windows) I recommend
putting the file in your path so you can do a simple command line call `srm`.

The easier option, however is to install with Cargo. Simply run the command:

```bash
cargo install secure_remove
```

This will download the crate from crates.io, build, and automatically add the executable to your path.

## Installing from Source

You can also download the source code and install from scratch if you would like. You will
need cargo installed on your system.

```bash
git clone https://github.com/saizo80/Secure-Remove.git
cargo install --path Secure-Remove
```

## Usage

The simplest scheme of using srm is `srm [Options] [Target(s)]`.

The functions are very similar to what you can do with regular `rm`, except (for now) you cannot use wildcards inside of a path.

So `srm ./*` would be okay, whereas `srm ./*.txt` would **not** be okay.

Other functions you can see by running `srm --help`.

## Considerations

I, the author, am not a security expert. *Therefore this project comes with no guarantees that file recovery will be **impossible**.*

However, I have tried, to the best of my ability, to make this a secure deletion program.

The data deleted by this program **cannot** be recovered by oridinary methods. Please make sure you know what you are doing being using.
