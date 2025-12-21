# Contributing to VietFlux IME

Cảm ơn bạn đã quan tâm đến việc đóng góp cho VietFlux! 🎉

## 📋 Mục lục

- [Code of Conduct](#code-of-conduct)
- [Cách đóng góp](#cách-đóng-góp)
- [Development Setup](#development-setup)
- [Coding Guidelines](#coding-guidelines)
- [Commit Convention](#commit-convention)
- [Pull Request Process](#pull-request-process)

---

## Code of Conduct

Dự án này tuân theo [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). Khi tham gia, bạn đồng ý tuân thủ các quy tắc này.

---

## 🚀 Cách đóng góp

### 🐛 Báo lỗi (Bug Reports)

1. Kiểm tra [Issues](https://github.com/ThanhNguyxn/vietflux-ime/issues) xem lỗi đã được báo chưa
2. Nếu chưa, tạo issue mới với template "Bug Report"
3. Cung cấp đầy đủ thông tin: OS, version, steps to reproduce

### ✨ Đề xuất tính năng

1. Mở issue với template "Feature Request"
2. Mô tả tính năng và lý do cần thiết
3. Thảo luận với maintainers trước khi code

### 💻 Đóng góp code

1. Fork repo
2. Tạo branch mới: `git checkout -b feature/TenTinhNang`
3. Code và test
4. Commit theo convention
5. Mở Pull Request

---

## 🛠️ Development Setup

### Prerequisites

- [Rust](https://rustup.rs/) 1.70+
- [wasm-pack](https://rustwasm.github.io/wasm-pack/)
- [Node.js](https://nodejs.org/) 18+ (cho web demo)

### Clone và Setup

```bash
# Clone repo
git clone https://github.com/ThanhNguyxn/vietflux-ime.git
cd vietflux-ime

# Build core engine
cd core
cargo build

# Run tests
cargo test

# Build WASM
wasm-pack build --target web
```

### Project Structure

```
vietflux-ime/
├── core/               # Rust core engine
│   ├── src/
│   │   ├── lib.rs      # WASM bindings
│   │   ├── engine.rs   # Main engine
│   │   ├── buffer.rs   # Input buffer
│   │   ├── chars.rs    # Character data
│   │   ├── transform.rs# Transformations
│   │   ├── validation.rs # Validation
│   │   ├── shortcuts.rs# Shortcuts
│   │   └── methods/    # Input methods
│   └── Cargo.toml
├── web/                # Web demo
├── .github/            # GitHub workflows
└── README.md
```

---

## 📝 Coding Guidelines

### Rust

- Format: `cargo fmt`
- Lint: `cargo clippy`
- Test: `cargo test`
- Docs: Comment public functions với `///`

```rust
/// Apply tone mark to a vowel character.
/// 
/// # Arguments
/// * `ch` - The character to modify
/// * `tone` - The tone mark to apply
/// 
/// # Returns
/// The modified character, or None if invalid
pub fn apply_tone(ch: char, tone: ToneMark) -> Option<char> {
    // implementation
}
```

### Naming Conventions

| Type | Convention | Example |
|------|------------|---------|
| Functions | snake_case | `apply_tone()` |
| Structs | PascalCase | `Engine` |
| Constants | SCREAMING_SNAKE | `VALID_INITIALS` |
| Files | snake_case | `validation.rs` |

---

## 📦 Commit Convention

Sử dụng [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

### Types

| Type | Mô tả |
|------|-------|
| `feat` | Tính năng mới |
| `fix` | Sửa lỗi |
| `docs` | Documentation |
| `style` | Formatting, không thay đổi logic |
| `refactor` | Refactor code |
| `test` | Thêm/sửa tests |
| `chore` | Build, CI, dependencies |

### Ví dụ

```
feat(engine): add UO compound handling

- Implement ươ compound detection
- Tone now correctly goes on ơ
- Add tests for edge cases

Closes #29
```

---

## 🔄 Pull Request Process

1. **Trước khi tạo PR:**
   - [ ] Code passes `cargo fmt`
   - [ ] Code passes `cargo clippy`
   - [ ] All tests pass `cargo test`
   - [ ] Đã thêm tests cho code mới

2. **Tạo PR:**
   - Sử dụng template có sẵn
   - Link đến issue liên quan
   - Mô tả changes rõ ràng

3. **Review:**
   - Ít nhất 1 approval từ maintainer
   - CI phải pass
   - Không có merge conflicts

4. **Merge:**
   - Squash and merge cho feature branches
   - Rebase and merge cho bugfixes

---

## 🏷️ Issue Labels

| Label | Mô tả |
|-------|-------|
| `bug` | Lỗi cần sửa |
| `enhancement` | Tính năng mới |
| `documentation` | Docs cần cập nhật |
| `good first issue` | Phù hợp cho newcomers |
| `help wanted` | Cần sự giúp đỡ |
| `priority: high` | Ưu tiên cao |
| `wontfix` | Sẽ không fix |

---

## ❓ Câu hỏi?

- Mở [Discussion](https://github.com/ThanhNguyxn/vietflux-ime/discussions)

---

Cảm ơn bạn đã đóng góp! 🙏
