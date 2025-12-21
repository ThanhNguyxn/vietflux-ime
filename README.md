<p align="center">
  <img src="https://img.shields.io/badge/🇻🇳-VietFlux_IME-blue?style=for-the-badge" alt="VietFlux IME"/>
</p>

<h1 align="center">⚡ VietFlux IME</h1>

<p align="center">
  <strong>Bộ gõ tiếng Việt thông minh - Dành cho Developers & Everyone</strong>
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
  <a href="#-phím-tắt">Phím tắt</a> •
  <a href="#-support">Ủng hộ</a>
</p>

---

## 🎯 Tại sao chọn VietFlux?

| Vấn đề với bộ gõ cũ | VietFlux giải quyết |
|---------------------|---------------------|
| ❌ Gõ code bị biến thành tiếng Việt (`neư` thay vì `new`) | ✅ **Tự nhận diện English** - Code thoải mái! |
| ❌ Chậm, lag khi gõ nhanh | ✅ **Siêu nhanh** - Viết bằng Rust, < 1ms/phím |
| ❌ Không chạy trên Web/Cross-platform | ✅ **Chạy mọi nơi** - Windows, macOS, Linux, Web |
| ❌ Gõ `được` ra `đưọc` | ✅ **Smart ươ** - Tự động đặt dấu đúng chỗ |
| ❌ Phải tắt/bật IME liên tục | ✅ **Thông minh** - Tự biết khi nào dùng |

---

## 📦 Cài đặt

### 🪟 Windows

**Cách 1: Cài 1-click (Khuyến nghị)**
```
1. Tải file: VietFlux-Setup.exe
2. Double-click để cài
3. Xong! Bắt đầu gõ tiếng Việt
```

**Cách 2: Portable (Không cần cài)**
```
1. Tải file: VietFlux-Portable.zip
2. Giải nén ra thư mục bất kỳ
3. Chạy VietFlux.exe
```

> 📥 **[Tải về cho Windows](https://github.com/ThanhNguyxn/vietflux-ime/releases/latest)**

---

### 🍎 macOS

**Cách 1: Homebrew (Khuyến nghị)**
```bash
brew install --cask vietflux
```

**Cách 2: Cài thủ công**
```
1. Tải file: VietFlux.dmg
2. Mở file .dmg
3. Kéo VietFlux vào thư mục Applications
4. Mở VietFlux từ Applications
5. Cho phép trong System Settings → Privacy → Accessibility
```

> 📥 **[Tải về cho macOS](https://github.com/ThanhNguyxn/vietflux-ime/releases/latest)**

---

### 🐧 Linux

**Ubuntu/Debian:**
```bash
# Thêm repo
curl -fsSL https://vietflux.dev/gpg | sudo gpg --dearmor -o /usr/share/keyrings/vietflux.gpg
echo "deb [signed-by=/usr/share/keyrings/vietflux.gpg] https://vietflux.dev/apt stable main" | sudo tee /etc/apt/sources.list.d/vietflux.list

# Cài đặt
sudo apt update && sudo apt install vietflux
```

**Fedora/RHEL:**
```bash
sudo dnf install vietflux
```

**Arch Linux:**
```bash
yay -S vietflux
```

**AppImage (Chạy trên mọi distro):**
```bash
# Tải và chạy
wget https://github.com/ThanhNguyxn/vietflux-ime/releases/latest/download/VietFlux.AppImage
chmod +x VietFlux.AppImage
./VietFlux.AppImage
```

> 📥 **[Tải về cho Linux](https://github.com/ThanhNguyxn/vietflux-ime/releases/latest)**

---

### 🌐 Web Demo (Dùng thử ngay!)

Không muốn cài? Thử trực tiếp trên trình duyệt:

👉 **[Dùng thử VietFlux Online](https://vietflux.dev/demo)**

---

## ✨ Tính năng

### 🚀 Core Features

| Tính năng | Mô tả |
|-----------|-------|
| ⚡ **Siêu nhanh** | Core engine viết bằng Rust, < 1ms mỗi phím |
| 🎯 **Telex & VNI** | Hỗ trợ cả hai phương thức gõ phổ biến |
| 🌐 **Cross-platform** | Windows, macOS, Linux, và Web |
| 📦 **Siêu nhẹ** | Chỉ ~3MB, không làm nặng máy |

### 🧠 Smart Features (Dành cho Developers)

| Tính năng | Mô tả |
|-----------|-------|
| 🔍 **Tự nhận diện English** | `neư` → tự restore thành `new` |
| 🎯 **Smart ươ Compound** | `dduwocj` → `được` (không phải `đưọc`) |
| ⏪ **Double Mark Undo** | Gõ `as` + `s` → `á`, gõ `s` nữa → `as` |
| 📝 **Shortcut Expansion** | `ko` → `không`, `dc` → `được` |
| 🔄 **Auto-restore** | Gõ sai tự động sửa khi nhấn Space |

### 🛡️ Validation Features

| Tính năng | Mô tả |
|-----------|-------|
| ✅ **5 Phonology Rules** | Kiểm tra âm đầu, âm cuối, spelling rules |
| 🚫 **Invalid Pattern Detection** | Phát hiện `eư`, `oư`, `iư` (không hợp lệ) |
| 🔤 **Typing Sequence Aware** | `dodo` = đang gõ `đô`, không restore |

### 🎨 UX Features

| Tính năng | Mô tả |
|-----------|-------|
| 🌙 **Dark Mode** | Giao diện tối hiện đại |
| 🖥️ **System Tray** | Chạy nền, không chiếm taskbar |
| ⌨️ **Hotkey Toggle** | Bật/tắt nhanh bằng phím tắt |

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

### Phím tắt hệ thống

| Phím | Chức năng |
|------|-----------|
| `Ctrl + Shift` | Bật/tắt VietFlux |
| `Ctrl + .` | Chuyển Telex ↔ VNI |

---

## 🏗️ Kiến trúc

```
┌────────────────────────────────────────────────────────┐
│                    VietFlux IME                         │
├─────────┬─────────┬─────────┬─────────┬───────────────┤
│ Windows │  macOS  │  Linux  │   Web   │    Mobile     │
│  Native │ Native  │ Native  │  WASM   │   (Coming)    │
├─────────┴─────────┴─────────┴─────────┴───────────────┤
│                 WebAssembly (WASM)                      │
├────────────────────────────────────────────────────────┤
│                  Rust Core Engine                       │
│  ┌──────────┐ ┌──────────┐ ┌────────────────────────┐  │
│  │ Validate │ │ Transform│ │   Smart Detection      │  │
│  │ Phonology│ │ Telex/VNI│ │   English/Vietnamese   │  │
│  └──────────┘ └──────────┘ └────────────────────────┘  │
└────────────────────────────────────────────────────────┘
```

---

## 🛠️ Công nghệ

<p align="center">
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust"/>
  <img src="https://img.shields.io/badge/WebAssembly-654FF0?style=for-the-badge&logo=webassembly&logoColor=white" alt="WebAssembly"/>
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

## ☕ Support

Nếu bạn thấy dự án này hữu ích, hãy ủng hộ tác giả:

<p align="center">
  <a href="https://buymeacoffee.com/thanhnguyxn">
    <img src="https://img.shields.io/badge/Buy_Me_A_Coffee-FFDD00?style=for-the-badge&logo=buy-me-a-coffee&logoColor=black" alt="Buy Me A Coffee"/>
  </a>
  <a href="https://github.com/sponsors/ThanhNguyxn">
    <img src="https://img.shields.io/badge/GitHub_Sponsors-EA4AAA?style=for-the-badge&logo=github-sponsors&logoColor=white" alt="GitHub Sponsors"/>
  </a>
</p>

---

<p align="center">
  Made with ❤️ in Vietnam 🇻🇳
</p>

<p align="center">
  <a href="https://github.com/ThanhNguyxn/vietflux-ime">
    <img src="https://img.shields.io/badge/⭐_Star_this_repo-yellow?style=for-the-badge" alt="Star"/>
  </a>
</p>
