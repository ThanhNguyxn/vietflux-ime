<p align="center">
  <img src="app/src-tauri/icons/icon.png" alt="VietFlux Logo" width="120"/>
</p>

<h1 align="center">⚡ VietFlux IME</h1>

<p align="center">
  <strong>Bộ gõ tiếng Việt hiệu năng cao - Native Desktop App</strong>
</p>

<p align="center">
  <a href="https://github.com/ThanhNguyxn/vietflux-ime/releases/latest">
    <img src="https://img.shields.io/github/v/release/ThanhNguyxn/vietflux-ime?style=for-the-badge&logo=github&color=blue" alt="Release"/>
  </a>
  <a href="LICENSE">
    <img src="https://img.shields.io/badge/License-MIT-green?style=for-the-badge" alt="License"/>
  </a>
  <a href="https://github.com/ThanhNguyxn/vietflux-ime/stargazers">
    <img src="https://img.shields.io/github/stars/ThanhNguyxn/vietflux-ime?style=for-the-badge&logo=github&color=yellow" alt="Stars"/>
  </a>
</p>

<p align="center">
  <a href="https://github.com/ThanhNguyxn/vietflux-ime/releases">
    <img src="https://img.shields.io/badge/Windows-0078D6?style=flat-square&logo=windows&logoColor=white" alt="Windows"/>
  </a>
  <a href="https://github.com/ThanhNguyxn/vietflux-ime/releases">
    <img src="https://img.shields.io/badge/macOS-000000?style=flat-square&logo=apple&logoColor=white" alt="macOS"/>
  </a>
  <a href="https://github.com/ThanhNguyxn/vietflux-ime/releases">
    <img src="https://img.shields.io/badge/Linux-FCC624?style=flat-square&logo=linux&logoColor=black" alt="Linux"/>
  </a>
</p>

<p align="center">
  <a href="#-tại-sao-vietflux">Tại sao?</a> •
  <a href="#-cài-đặt">Cài đặt</a> •
  <a href="#-tính-năng">Tính năng</a> •
  <a href="#-phím-tắt">Phím tắt</a> •
  <a href="#-ủng-hộ">Ủng hộ</a>
</p>

---

## 🎯 Tại sao VietFlux?

| Vấn đề với bộ gõ cũ | VietFlux giải quyết |
|---------------------|---------------------|
| ❌ Cài đặt phức tạp | ✅ **1-click install** - Tải, cài, xong! |
| ❌ Chậm, lag khi gõ nhanh | ✅ **Siêu nhanh** - Core viết bằng Rust |
| ❌ Chỉ chạy trên 1 OS | ✅ **Windows + macOS + Linux** |
| ❌ Gõ `được` ra `đưọc` | ✅ **Smart ươ** - Đặt dấu đúng vị trí |
| ❌ Gõ `new` ra `neư` | ✅ **Nhận diện English** - Tự động restore |

---

## 📦 Cài đặt

### 📥 Tải về

<p align="center">
  <a href="https://github.com/ThanhNguyxn/vietflux-ime/releases/latest">
    <img src="https://img.shields.io/badge/📥_Tải_Về-VietFlux-blue?style=for-the-badge&logo=github" alt="Download"/>
  </a>
</p>

| Hệ điều hành | File | Cách cài |
|:------------:|:----:|:--------:|
| 🪟 **Windows** | `.exe` / `.msi` | Double-click để cài |
| 🍎 **macOS Intel** | `x64.dmg` | Kéo vào Applications |
| 🍎 **macOS Apple Silicon** | `arm64.dmg` | Kéo vào Applications |
| 🐧 **Linux** | `.AppImage` / `.deb` | Chạy hoặc `dpkg -i` |

> 📖 **Hướng dẫn chi tiết:** Xem [docs/INSTALL.md](docs/INSTALL.md)

---

## ✨ Tính năng

| Tính năng | Mô tả |
|-----------|-------|
| ⚡ **Hiệu năng cao** | Core engine viết bằng Rust |
| 🎯 **Telex & VNI** | Hỗ trợ cả hai kiểu gõ phổ biến |
| 🌐 **Cross-platform** | Windows, macOS, Linux |
| 🧠 **Smart ươ Compound** | `dduwocj` → `được` (không phải `đưọc`) |
| 🔍 **Nhận diện English** | `neư` → tự restore thành `new` |
| ⏪ **Double Mark Undo** | Gõ `s` 2 lần để undo dấu sắc |
| 📝 **Shortcut Expansion** | `ko` → `không`, `dc` → `được` |
| 🖥️ **System Tray** | Chạy nền, không chiếm taskbar |

---

## ⌨️ Phím tắt

### Hệ thống

| Phím | Chức năng |
|:----:|:---------:|
| `Ctrl + Shift` | Bật/tắt VietFlux |
| `Ctrl + .` | Chuyển Telex ↔ VNI |

### Telex

| Phím | Kết quả | Ví dụ |
|:----:|:-------:|:-----:|
| `aa` | â | `caam` → cầm |
| `ee` | ê | `been` → bên |
| `oo` | ô | `coon` → côn |
| `aw` | ă | `tawn` → tăng |
| `ow` | ơ | `tow` → tơ |
| `uw` | ư | `tuw` → tư |
| `dd` | đ | `ddi` → đi |
| `s` | sắc ´ | `as` → á |
| `f` | huyền ` | `af` → à |
| `r` | hỏi ̉ | `ar` → ả |
| `x` | ngã ˜ | `ax` → ã |
| `j` | nặng ̣ | `aj` → ạ |

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

---

## 🏗️ Kiến trúc

```
┌─────────────────────────────────────────┐
│           VietFlux Desktop App          │
│        (Tauri - Rust + WebView)         │
├─────────────────────────────────────────┤
│              System Tray UI             │
├─────────────────────────────────────────┤
│            VietFlux Core Engine         │
│    Validation • Transform • Methods     │
└─────────────────────────────────────────┘
```

---

## 📚 Tài liệu

| Tài liệu | Mô tả |
|----------|-------|
| 📖 [INSTALL.md](docs/INSTALL.md) | Hướng dẫn cài đặt chi tiết |
| ❓ [FAQ.md](docs/FAQ.md) | Câu hỏi thường gặp |
| 🤝 [CONTRIBUTING.md](CONTRIBUTING.md) | Hướng dẫn đóng góp |
| 🔒 [SECURITY.md](SECURITY.md) | Chính sách bảo mật |
| 📜 [LICENSE](LICENSE) | MIT License |

---

## 🤝 Đóng góp

Mọi đóng góp đều được chào đón! 🎉

1. Fork repo này
2. Tạo branch: `git checkout -b feature/TenTinhNang`
3. Commit: `git commit -m 'feat: Thêm tính năng mới'`
4. Push: `git push origin feature/TenTinhNang`
5. Mở Pull Request

> 📖 Xem [CONTRIBUTING.md](CONTRIBUTING.md) để biết thêm chi tiết

---

## ☕ Ủng hộ

Nếu VietFlux hữu ích với bạn, hãy cân nhắc ủng hộ tác giả:

<p align="center">
  <a href="https://buymeacoffee.com/thanhnguyxn">
    <img src="https://img.shields.io/badge/Buy_Me_A_Coffee-FFDD00?style=for-the-badge&logo=buy-me-a-coffee&logoColor=black" alt="Buy Me A Coffee"/>
  </a>
  <a href="https://github.com/sponsors/ThanhNguyxn">
    <img src="https://img.shields.io/badge/GitHub_Sponsors-EA4AAA?style=for-the-badge&logo=github-sponsors&logoColor=white" alt="GitHub Sponsors"/>
  </a>
</p>

<p align="center">
  Mỗi đóng góp đều giúp dự án phát triển tốt hơn! 💪
</p>

---

## 🙏 Credits

- Inspired by [UniKey](https://www.unikey.org/) - Bộ gõ tiếng Việt phổ biến nhất

---

<p align="center">
  Made with ❤️ in Vietnam 🇻🇳
</p>

<p align="center">
  <a href="https://github.com/ThanhNguyxn/vietflux-ime">
    <img src="https://img.shields.io/badge/⭐_Star_this_repo-yellow?style=for-the-badge" alt="Star"/>
  </a>
</p>
