# whoami

In case I ever need to prove who I am down the line, I've made a small program.

## Dependencies

To use this program, you need to have rust and cargo installed. See: [https://www.rust-lang.org/learn/get-started](https://www.rust-lang.org/learn/get-started)

## Usage

### Method 1: Cloning the repository and building from source

To use the program, first clone the repository with:

```
git clone https://github.com/keyrowed/whoami.git
```

Then, run the following command from the root directory of the repository:

```
cargo run --release -- --input <your input>
```

### Method 2: Installing the program on your system

Alternatively, you can install this program and then use it by doing the following:

```
cargo install --git https://github.com/keyrowed/whoami.git
wai --input <your input>
```

To uninstall the program, run:

```
cargo uninstall whoami
```
