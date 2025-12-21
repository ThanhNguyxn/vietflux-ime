<p align="center">
  <img src="https://img.shields.io/badge/🇻🇳-VietFlux_IME-blue?style=for-the-badge" alt="VietFlux IME"/>
</p>

<h1 align="center">⚡ VietFlux IME</h1>

<p align="center">
  <strong>Bộ gõ tiếng Việt thông minh chạy trên trình duyệt</strong>
</p>

<p align="center">
  <a href="#-cài-đặt"><img src="https://img.shields.io/badge/Windows-0078D6?style=flat-square&logo=windows&logoColor=white" alt="Windows"/></a>
  <a href="#-cài-đặt"><img src="https://img.shields.io/badge/macOS-000000?style=flat-square&logo=apple&logoColor=white" alt="macOS"/></a>
  <a href="#-cài-đặt"><img src="https://img.shields.io/badge/Linux-FCC624?style=flat-square&logo=linux&logoColor=black" alt="Linux"/></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-green.svg?style=flat-square" alt="License: MIT"/></a>
</p>

<p align="center">
  <a href="#-tại-sao-chọn-vietflux">Tại sao?</a> •
  <a href="#-cài-đặt">Cài đặt</a> •
  <a href="#-tính-năng">Tính năng</a> •
  <a href="#-phím-tắt">Phím tắt</a>
</p>

---

## 🎯 Tại sao chọn VietFlux?

| Vấn đề với bộ gõ cũ | VietFlux giải quyết |
|---------------------|---------------------|
| ❌ Phải cài đặt phức tạp | ✅ **Chạy ngay** - Chỉ cần mở file HTML! |
| ❌ Chậm, lag khi gõ nhanh | ✅ **Siêu nhanh** - Viết bằng Rust + WASM |
| ❌ Chỉ chạy trên 1 OS | ✅ **Cross-platform** - Chạy mọi nơi có browser |
| ❌ Gõ `được` ra `đưọc` | ✅ **Smart ươ** - Tự động đặt dấu đúng chỗ |

---

## 📦 Cài đặt

### Bước 1: Tải về

👉 **[Tải VietFlux tại đây](https://github.com/ThanhNguyxn/vietflux-ime/releases/latest)**

| Hệ điều hành | File tải |
|--------------|----------|
| 🪟 Windows | `VietFlux-*-Windows.zip` |
| 🍎 macOS / 🐧 Linux | `VietFlux-*-Linux-macOS.tar.gz` |

### Bước 2: Giải nén

**Windows:** Click chuột phải → Extract All

**macOS/Linux:**
```bash
tar -xzf VietFlux-*-Linux-macOS.tar.gz
```

### Bước 3: Chạy

**Windows:** Double-click `Run-VietFlux.bat` hoặc mở `index.html`

**macOS/Linux:** 
```bash
./run-vietflux.sh
# hoặc mở index.html trong browser
```

**Xong!** Bắt đầu gõ tiếng Việt! 🎉

---

## ✨ Tính năng

| Tính năng | Mô tả |
|-----------|-------|
| ⚡ **Siêu nhanh** | Core engine viết bằng Rust + WebAssembly |
| 🎯 **Telex & VNI** | Hỗ trợ cả hai phương thức gõ phổ biến |
| 🌐 **Cross-platform** | Windows, macOS, Linux - chỉ cần browser |
| 🧠 **Smart ươ** | Tự động đặt dấu đúng vị trí trong `ươ` |
| 🔍 **Nhận diện English** | Từ như `new` không bị biến thành `neư` |

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
┌────────────────────────────────────────┐
│           VietFlux IME                  │
├─────────┬─────────┬─────────┬──────────┤
│ Windows │  macOS  │  Linux  │   Web    │
│         (Chạy trên Browser)             │
├─────────────────────────────────────────┤
│            WebAssembly (WASM)           │
├─────────────────────────────────────────┤
│           Rust Core Engine              │
└─────────────────────────────────────────┘
```

---

## 🤝 Đóng góp

Xem [CONTRIBUTING.md](CONTRIBUTING.md) để biết cách đóng góp.

---

## 📝 License

MIT License - Xem [LICENSE](LICENSE)

---

## 🙏 Credits

Inspired by [UniKey](https://www.unikey.org/)

---

<p align="center">
  Made with ❤️ in Vietnam 🇻🇳
</p>
