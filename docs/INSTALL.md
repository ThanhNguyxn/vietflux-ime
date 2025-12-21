# 📦 Hướng dẫn cài đặt VietFlux IME

Chọn hệ điều hành của bạn:

- [🪟 Windows](#windows)
- [🍎 macOS](#macos)
- [🐧 Linux](#linux)

---

## 🪟 Windows

### Cách 1: Installer (Khuyến nghị)

1. **Tải về:** [VietFlux-Setup.exe](https://github.com/ThanhNguyxn/vietflux-ime/releases/latest)
2. **Chạy file** đã tải
3. **Làm theo hướng dẫn** trên màn hình
4. **Xong!** VietFlux sẽ tự khởi động

### Cách 2: Portable (Không cần cài)

1. **Tải về:** [VietFlux-Portable.zip](https://github.com/ThanhNguyxn/vietflux-ime/releases/latest)
2. **Giải nén** vào thư mục bất kỳ
3. **Chạy** `VietFlux.exe`

### Gỡ cài đặt

- **Installer:** Settings → Apps → VietFlux → Uninstall
- **Portable:** Xóa thư mục đã giải nén

---

## 🍎 macOS

### Cách 1: Homebrew (Khuyến nghị)

```bash
brew install --cask vietflux
```

### Cách 2: Tải file DMG

1. **Tải về:** [VietFlux.dmg](https://github.com/ThanhNguyxn/vietflux-ime/releases/latest)
2. **Mở file** DMG
3. **Kéo** VietFlux vào thư mục **Applications**
4. **Mở** VietFlux từ Applications
5. **Cho phép** trong System Settings → Privacy & Security → Accessibility

### Lần đầu mở

macOS có thể báo "unidentified developer". Làm theo:

1. Click chuột phải vào app → **Open**
2. Click **Open** trong dialog

### Gỡ cài đặt

```bash
# Homebrew
brew uninstall vietflux

# Manual
rm -rf /Applications/VietFlux.app
rm -rf ~/Library/Preferences/dev.vietflux.*
rm -rf ~/Library/Application\ Support/VietFlux
```

---

## 🐧 Linux

### Ubuntu/Debian

```bash
# Thêm repository
sudo curl -fsSL https://vietflux.dev/apt/gpg.key -o /usr/share/keyrings/vietflux.gpg
echo "deb [signed-by=/usr/share/keyrings/vietflux.gpg] https://vietflux.dev/apt stable main" | sudo tee /etc/apt/sources.list.d/vietflux.list

# Cài đặt
sudo apt update
sudo apt install vietflux
```

### Fedora/RHEL

```bash
sudo dnf copr enable thanhnguyxn/vietflux
sudo dnf install vietflux
```

### Arch Linux (AUR)

```bash
yay -S vietflux
# hoặc
paru -S vietflux
```

### AppImage (Chạy mọi distro)

```bash
# Tải về
wget https://github.com/ThanhNguyxn/vietflux-ime/releases/latest/download/VietFlux.AppImage

# Cho phép chạy
chmod +x VietFlux.AppImage

# Chạy
./VietFlux.AppImage
```

### Gỡ cài đặt

```bash
# Ubuntu/Debian
sudo apt remove vietflux

# Fedora
sudo dnf remove vietflux

# AppImage
rm VietFlux.AppImage
```

---

## ❓ Gặp vấn đề?

- [Xem FAQ](FAQ.md)
- [Báo lỗi](https://github.com/ThanhNguyxn/vietflux-ime/issues/new?template=bug_report.yml)
