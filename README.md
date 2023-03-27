# Awqat - أوقات الصلاة

Simple CLI Application that fetches Salah times from salah.com

## Usage

```
awqat - Simple CLI Application that fetches Salah times from salah.com

Usage: awqat [OPTIONS]

Options:
-V, --version       Print version info
-S, --show-location Run with showing the location
    --help          Show this help message
```

### Installation

Run `install.sh` that does it all for you:

```sh
git clone https://github.com/QurashiAkh/awqat.git
cd awqat
./install.sh
```

or do it yourself:

Build `awqat`:

```sh
cargo build --release
```

Then copy the binary file to `/usr/local/bin/`:

```sh
sudo cp ./target/release/awqat /usr/local/bin/awqat
```

Now you can use it by running `awqat` in your Terminal.

### Removal

Delete the `/usr/local/bin/awqat` file, or `cd` into the repo and run:

```sh
./uninstall.sh
```

## Contributing

Contributions to this Project are welcomed. Feel free to make a pull request if you have anything to suggest.

## LICENSE

Awqat is licensed under the GNU General Public License 3.0.
