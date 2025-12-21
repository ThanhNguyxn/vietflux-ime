<p align="center">
  <img src="https://img.shields.io/badge/🇻🇳-VietFlux_IME-blue?style=for-the-badge" alt="VietFlux IME"/>
</p>

<h1 align="center">⚡ VietFlux IME</h1>

<p align="center">
  <strong>Bộ gõ tiếng Việt native cho Windows, macOS, Linux</strong>
</p>

<p align="center">
  <a href="#-cài-đặt"><img src="https://img.shields.io/badge/Windows-0078D6?style=flat-square&logo=windows&logoColor=white" alt="Windows"/></a>
  <a href="#-cài-đặt"><img src="https://img.shields.io/badge/macOS-000000?style=flat-square&logo=apple&logoColor=white" alt="macOS"/></a>
  <a href="#-cài-đặt"><img src="https://img.shields.io/badge/Linux-FCC624?style=flat-square&logo=linux&logoColor=black" alt="Linux"/></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-green.svg?style=flat-square" alt="License: MIT"/></a>
</p>

---

## 🎯 Tại sao chọn VietFlux?

| Vấn đề | VietFlux |
|--------|----------|
| ❌ Phải cài đặt phức tạp | ✅ **1-click install** - Tải về, cài, xong! |
| ❌ Chậm, lag | ✅ **Siêu nhanh** - Viết bằng Rust |
| ❌ Chỉ 1 OS | ✅ **Windows + macOS + Linux** |
| ❌ Gõ `được` ra `đưọc` | ✅ **Smart ươ** - Đặt dấu đúng |

---

## 📦 Cài đặt

### 1️⃣ Tải về

👉 **[Tải VietFlux](https://github.com/ThanhNguyxn/vietflux-ime/releases/latest)**

| OS | File |
|----|------|
| 🪟 Windows | `.exe` hoặc `.msi` |
| 🍎 macOS Intel | `x64.dmg` |
| 🍎 macOS Apple Silicon | `arm64.dmg` |
| 🐧 Linux | `.AppImage` hoặc `.deb` |

### 2️⃣ Cài đặt

- **Windows**: Double-click file `.exe` hoặc `.msi`
- **macOS**: Mở `.dmg`, kéo vào Applications
- **Linux**: Chạy `.AppImage` hoặc `sudo dpkg -i *.deb`

### 3️⃣ Sử dụng

VietFlux chạy trong **khay hệ thống** (system tray). Click icon để mở settings.

---

## ⌨️ Phím tắt

### Hệ thống
| Phím | Chức năng |
|------|-----------|
| `Ctrl + Shift` | Bật/tắt VietFlux |

### Telex
| Phím | Kết quả |
|:----:|:-------:|
| `aa` | â |
| `aw` | ă |
| `ow` | ơ |
| `uw` | ư |
| `dd` | đ |
| `s/f/r/x/j` | sắc/huyền/hỏi/ngã/nặng |

### VNI
| Phím | Kết quả |
|:----:|:-------:|
| `1-5` | sắc/huyền/hỏi/ngã/nặng |
| `6/7/8` | mũ/móc/trăng |
| `9` | đ |

---

## 🏗️ Kiến trúc

```
┌──────────────────────────────────┐
│         VietFlux App             │
│     (Tauri - Rust + WebView)     │
├──────────────────────────────────┤
│         System Tray UI           │
├──────────────────────────────────┤
│        VietFlux Core             │
│    (Rust Engine + Validation)    │
└──────────────────────────────────┘
```

---

## 🤝 Đóng góp

Xem [CONTRIBUTING.md](CONTRIBUTING.md)

---

## 📝 License

MIT - Xem [LICENSE](LICENSE)

---

<p align="center">Made with ❤️ in Vietnam 🇻🇳</p>
