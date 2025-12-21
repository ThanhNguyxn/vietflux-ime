# GitHub Actions Workflows Logic

## 1. CI (Continuous Integration)
**File:** `.github/workflows/ci.yml`
**Mục đích:** Đảm bảo code luôn "sạch" và chạy đúng trên mọi hệ điều hành mỗi khi có thay đổi.

### 🎯 Trigger (Khi nào chạy?)
- **Push** vào nhánh `main`.
- **Pull Request** vào nhánh `main`.
- **Điều kiện:** Chỉ chạy khi có thay đổi trong:
    - `core/**` (Code lõi)
    - `app/**` (App giao diện)
    - `Cargo.toml` (Cấu hình)
    - `.github/workflows/**` (File cấu hình CI)

### ⚙️ Các Job (Làm gì?)
1.  **🔍 Lint**:
    - Chạy `cargo fmt`: Kiểm tra format code cho đẹp.
    - Chạy `cargo clippy`: Bắt các lỗi tiềm ẩn hoặc code thừa.
2.  **🧪 Test (Matrix)**:
    - Chạy `cargo test` trên 3 môi trường cùng lúc:
        - **Ubuntu** (Linux)
        - **Windows**
        - **macOS**
    - Đảm bảo code chạy ngon trên cả 3 hệ điều hành.
3.  **🕸️ Build WASM**:
    - Thử build ra file `.wasm` để chắc chắn bản Web vẫn hoạt động.
4.  **📊 Coverage**:
    - Tính toán độ bao phủ test (bao nhiêu % code đã được test).

---

## 2. Release (Build & Publish)
**File:** `.github/workflows/release.yml`
**Mục đích:** Tự động build ra file cài đặt (.exe, .dmg) và đăng lên GitHub Releases.

### 🎯 Trigger (Khi nào chạy?)
- Khi bạn push một **Tag** bắt đầu bằng chữ `v` (ví dụ: `v1.2.0`, `v2.0.0`).

### ⚙️ Các Job (Làm gì?)
1.  **📦 Build Desktop App (Matrix)**:
    - Chạy song song trên 4 môi trường:
        - **Windows** (`.exe`, `.msi`)
        - **Linux** (`.AppImage`, `.deb`)
        - **macOS Intel** (`.dmg`)
        - **macOS Apple Silicon** (`.dmg`)
    - **🤖 Bước Tự Động Hóa (Mới thêm)**:
        - Lấy version từ Tag (ví dụ `v1.2.0` -> `1.2.0`).
        - Tự động sửa `Cargo.toml` và `tauri.conf.json` thành version này trước khi build.
    - Upload file cài đặt lên Artifacts.

2.  **🚀 Create Release**:
    - Đợi tất cả các job Build xong.
    - Tải tất cả file cài đặt về.
    - Tạo một **GitHub Release** mới với tên Tag.
    - Đính kèm các file cài đặt vào Release đó.
