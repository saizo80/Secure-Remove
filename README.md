# Secure Remove CLI

A terminal program for securely removing files with sensitive data, written in Rust.

While using the Eraser software for windows, I needed to be able to securely erase sensitive files
from the terminal, and thus be able to call it in scripts. So I decided to write
my own application to serve that purpose. I originally wrote the script in C#, but decided to move
to Rust for the speed benefits.

## Installation

Simply select your architecture from the latest release and unzip the archive.

You will have the file named `srm` (`srm.exe` on Windows) I recommend
putting the file in your path so you can do a simple command line call `srm`.

If you have Cargo installed on your system, however, the easiest way of installing
would be to run `cargo install --git https://github.com/saizo80/Secure-Remove`.

This will download, build, and automatically add the executable to your path.

## Building from Source

You can download the source code and build from scratch if you would like. You will
need cargo installed on your system.

```bash
git clone https://github.com/saizo80/Secure-Remove.git
cd Secure-Remove
cargo build -r
```

## Usage

The simplest scheme of using srm is `srm [Options] [Target]`.

The functions are very similar to what you can do with regular `rm`, except (for now) you cannot use wildcards inside of a path.

So `srm ./*` would be okay, whereas `srm ./*.txt` would **not** be okay.

Other functions you can see by running `srm --help`.