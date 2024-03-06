# Rustic Storage Delta 🛠️

### A minimalistic clone of the [storage-delta](https://github.com/0xPolygon/storage-delta) library written in Rust.

> [!WARNING]
> This project is created solely for fun and educational purposes. It is not intended to be used in any industrial, commercial, or production environment.

## Install

1. **Navigate to your `Foundry` project directory.**

2. **In your project directory, navigate to the `lib` folder:**

```bash
cd lib
```

3. **Clone the repository:**

```bash
git clone https://github.com/kamuik16/rustic-storage-delta
```

4. **Navigate to the rustic-storage-delta directory:**

```bash
cd rustic-storage-delta
```

5. **Execute the cargo build command:**

```bash
cargo build
```

## Run

From the root of your project directory, execute the following command to run Rustic Storage Delta:

```bash
lib/rustic-storage-delta/target/debug/rustic-storage-delta <repo_url>
```

Replace `repo_url` with the URL of the repository you want to analyze for storage changes.

After running the command, `./rustic_storage_delta` will be generated if there are findings. Open `OLD` and `NEW` files side by side for the best experience.
