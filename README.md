# ToDo CLI App

![](https://img.shields.io/badge/Rust-black?style=for-the-badge&logo=rust&logoColor=white)
![](https://img.shields.io/badge/linux-blue?style=for-the-badge)

# Install
```bash
git clone https://github.com/av1ppp/rust-todo && cd rust-todo
cargo build --release
sudo cp target/release/todo /bin/todo
cd .. && rm -rf rust-todo
```

# Using
| Command          | Description       |
|------------------|-------------------|
| `todo get`       | Print all todos   |
| `todo get <id>`  | Print todo by id  |
| `todo add <msg>` | Add todo          |
| `todo del <id>`  | Delete todo by id |
