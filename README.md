# Secure Remove CLI

A terminal program for securely removing files with sensitive data, written in C#.

While using the Eraser software for windows, I needed to be able to securely erase sensitive files
from the terminal, and thus be able to call it in scripts. So I decided to write
my own application to serve that purpose.

## Installation

Simply select your architecture from the latest release and unzip the archive. Please note executables in the release tab
are built to be self-contained. This means that they do not need to have .NET installed on the machine. However this increases
the file size from ~100-200KB to ~14-20MB. This is an unavoidable consequence of having it self-contained. If lower sizes
are needed I recommend building from source rather than downloading a pre-built binary.

You will have two files, `srm` (`srm.exe` on windows) and `srm.pdb`. I recommend
putting these files in your path so you can do a simple command line call `srm`.

## Building from Source

If you really want to build from source, there's nothing stopping you.

Firstly make sure that you have at least .NET 7.0 from Microsoft installed.
Which you can get from here: <https://dotnet.microsoft.com/en-us/download>

Just download the source code and use `dotnet publish -o Release` and it will build for your architecture.

Do **not** use the build scripts. They are for me and they will build and package for **all** architectures supported in the release, and the
binaries that are built will be the self-contained version.

## Usage

The simplest scheme of using srm is `srm [Options] [Target]`.

The functions are very similar to what you can do with regular `rm`, except (for now) you cannot use wildcards inside of a path.

So `srm ./*` would be okay, whereas `srm ./*.txt` would **not** be okay.

Other functions you can see by running `srm --help`.
