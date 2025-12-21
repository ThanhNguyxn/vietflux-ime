<p align="center">
  <img src="https://img.shields.io/badge/🇻🇳-VietFlux_IME-blue?style=for-the-badge" alt="VietFlux IME"/>
</p>

<h1 align="center">⚡ VietFlux IME</h1>

<p align="center">
  <strong>Bộ gõ tiếng Việt hiệu năng cao với WebAssembly</strong>
</p>

<p align="center">
  <a href="#features"><img src="https://img.shields.io/badge/Rust-000000?style=flat-square&logo=rust&logoColor=white" alt="Rust"/></a>
  <a href="#features"><img src="https://img.shields.io/badge/WebAssembly-654FF0?style=flat-square&logo=webassembly&logoColor=white" alt="WebAssembly"/></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-green.svg?style=flat-square" alt="License: MIT"/></a>
  <a href="https://github.com/ThanhNguyxn/vietflux-ime/stargazers"><img src="https://img.shields.io/github/stars/ThanhNguyxn/vietflux-ime?style=flat-square&color=yellow" alt="Stars"/></a>
</p>

<p align="center">
  <a href="#-tính-năng">Tính năng</a> •
  <a href="#-cài-đặt">Cài đặt</a> •
  <a href="#-sử-dụng">Sử dụng</a> •
  <a href="#-phím-tắt">Phím tắt</a> •
  <a href="#-đóng-góp">Đóng góp</a>
</p>

---

## ✨ Tính năng

| Tính năng | Mô tả |
|-----------|-------|
| ⚡ **Hiệu năng cao** | Core engine viết bằng Rust, compile to WebAssembly |
| 🎯 **Telex & VNI** | Hỗ trợ cả hai phương thức gõ phổ biến nhất |
| 🌐 **Cross-platform** | Chạy trên Web, Desktop (Tauri), và Mobile |
| 📦 **Siêu nhẹ** | Bundle size < 100KB gzipped |
| 🔒 **Privacy-first** | Xử lý hoàn toàn local, không gửi dữ liệu lên server |
| 🎨 **Modern UI** | Giao diện đẹp với Dark mode |

---

## 📁 Cấu trúc dự án

```
vietflux-ime/
├── 📂 core/                    # 🦀 Rust core engine
│   ├── src/
│   │   ├── lib.rs              # WASM bindings
│   │   ├── engine.rs           # Main IME engine
│   │   ├── buffer.rs           # Input buffer management
│   │   ├── chars.rs            # Vietnamese character data
│   │   ├── transform.rs        # Character transformation
│   │   ├── validation.rs       # Syllable validation
│   │   └── methods/            # Input methods
│   │       ├── telex.rs        # ⌨️ Telex
│   │       └── vni.rs          # 🔢 VNI
│   └── Cargo.toml
├── 📂 web/                     # 🌐 Web demo
│   └── index.html
├── 📄 README.md
└── 📄 LICENSE
```

---

## 🚀 Cài đặt

### Yêu cầu
- 🦀 [Rust](https://rustup.rs/) 1.70+
- 📦 [wasm-pack](https://rustwasm.github.io/wasm-pack/)

### Build từ source

```bash
# Clone repo
git clone https://github.com/ThanhNguyxn/vietflux-ime.git
cd vietflux-ime

# Build WASM
cd core
wasm-pack build --target web

# Chạy demo
cd ../web
python -m http.server 8080
```

---

## 💡 Sử dụng

### JavaScript/TypeScript

```javascript
import init, { VietFlux } from 'vietflux-core';

// Khởi tạo WASM
await init();

// Tạo instance IME
const ime = new VietFlux();

// Đặt phương thức gõ
ime.set_method('telex'); // hoặc 'vni'

// Xử lý phím
const result = ime.process_key('a', false);
console.log(result); // { action: "update", output: "a", backspace: 0 }

// Gõ "việt" bằng Telex
ime.process_key('v', false);
ime.process_key('i', false);
ime.process_key('e', false);
ime.process_key('e', false); // ee → ê
ime.process_key('j', false); // j → nặng
ime.process_key('t', false);
console.log(ime.get_buffer()); // "việt"
```

---

## ⌨️ Phím tắt

### Telex

| Phím | Kết quả | Ví dụ |
|:----:|:-------:|:-----:|
| `aa` | â | `caam` → cầm |
| `ee` | ê | `been` → bên |
| `oo` | ô | `coon` → côn |
| `aw` | ă | `awm` → ăm |
| `ow` | ơ | `tow` → tơ |
| `uw` | ư | `tuw` → tư |
| `dd` | đ | `ddi` → đi |
| `s` | sắc ´ | `as` → á |
| `f` | huyền ` | `af` → à |
| `r` | hỏi ̉ | `ar` → ả |
| `x` | ngã ˜ | `ax` → ã |
| `j` | nặng ̣ | `aj` → ạ |
| `z` | xóa dấu | `ász` → as |

### VNI

| Phím | Kết quả | Ví dụ |
|:----:|:-------:|:-----:|
| `1` | sắc ´ | `a1` → á |
| `2` | huyền ` | `a2` → à |
| `3` | hỏi ̉ | `a3` → ả |
| `4` | ngã ˜ | `a4` → ã |
| `5` | nặng ̣ | `a5` → ạ |
| `6` | mũ ^ | `a6` → â |
| `7` | móc ̛ | `o7` → ơ |
| `8` | trăng ̆ | `a8` → ă |
| `9` | đ | `d9` → đ |
| `0` | xóa dấu | `á0` → a |

---

## 🏗️ Kiến trúc

```
┌────────────────────────────────────────────────────────┐
│                    Applications                         │
├─────────┬─────────┬─────────┬─────────┬───────────────┤
│   Web   │  Tauri  │ Node.js │ Electron│    Native     │
│ Browser │ Desktop │  CLI    │   App   │  Windows/Mac  │
├─────────┴─────────┴─────────┴─────────┴───────────────┤
│                 WebAssembly (WASM)                      │
├────────────────────────────────────────────────────────┤
│                  Rust Core Engine                       │
│  ┌──────────┐ ┌──────────┐ ┌────────────────────────┐  │
│  │  Engine  │ │  Buffer  │ │      Transform         │  │
│  │          │ │          │ │   Telex │ VNI          │  │
│  └──────────┘ └──────────┘ └────────────────────────┘  │
└────────────────────────────────────────────────────────┘
```

---

## 🛠️ Công nghệ sử dụng

<p align="center">
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust"/>
  <img src="https://img.shields.io/badge/WebAssembly-654FF0?style=for-the-badge&logo=webassembly&logoColor=white" alt="WebAssembly"/>
  <img src="https://img.shields.io/badge/TypeScript-3178C6?style=for-the-badge&logo=typescript&logoColor=white" alt="TypeScript"/>
  <img src="https://img.shields.io/badge/Tauri-FFC131?style=for-the-badge&logo=tauri&logoColor=black" alt="Tauri"/>
</p>

---

## 🤝 Đóng góp

Mọi đóng góp đều được chào đón! 🎉

1. Fork repo này
2. Tạo branch mới (`git checkout -b feature/AmazingFeature`)
3. Commit changes (`git commit -m 'Add AmazingFeature'`)
4. Push to branch (`git push origin feature/AmazingFeature`)
5. Mở Pull Request

---

## 📝 License

Distributed under the **MIT License**. See [`LICENSE`](LICENSE) for more information.

---

## 🙏 Credits

- Inspired by [UniKey](https://www.unikey.org/) - Bộ gõ tiếng Việt phổ biến nhất Việt Nam

---

<p align="center">
  Made with ❤️ in Vietnam 🇻🇳
</p>

<p align="center">
  <a href="https://github.com/ThanhNguyxn/vietflux-ime">
    <img src="https://img.shields.io/badge/⭐_Star_this_repo-yellow?style=for-the-badge" alt="Star"/>
  </a>
</p>
