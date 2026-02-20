# Tauri + Leptos

Template này dùng để phát triển ứng dụng desktop với **Tauri (Rust backend)** và **Leptos (Rust frontend)**.

---

## 🚀 Development

Để bắt đầu phát triển, bạn cần mở **2 terminal** và chạy cả hai lệnh sau:

### 1️⃣ Terminal 1 – Build Tailwind CSS

```bash
npx tailwindcss -i ./styles.css -o ./styles.generated.css --watch
```
- Theo dõi file `styles.css`
- Tự động build ra `styles.generated.css` mỗi khi có thay đổi

---

### 2️⃣ Terminal 2 – Chạy Tauri App

```bash
cargo tauri dev
```
- Build backend Rust
- Khởi chạy ứng dụng desktop ở chế độ development

---

## 📌 Lưu ý quan trọng

Trong quá trình development, bạn **bắt buộc phải chạy đồng thời cả hai lệnh trên**:

- Một lệnh để build CSS
- Một lệnh để chạy ứng dụng

Nếu chỉ chạy một trong hai lệnh, giao diện sẽ không nhận được các class Tailwind mới hoặc app sẽ không khởi động đúng.

---

## 🧹 Format mã nguồn Leptos với leptosfmt

Để format code trong các macro `view!` của Leptos (giúp code sạch, dễ đọc hơn), bạn có thể sử dụng công cụ **leptosfmt**:

```bash
leptosfmt ./**/*.rs
```

- Lệnh này sẽ tự động format tất cả các file Rust trong dự án, bao gồm cả code bên trong macro `view!`.
- Đảm bảo bạn đã có `leptosfmt` trong môi trường phát triển (nếu dùng Nix, đã được cài sẵn qua `pkgs.leptosfmt`).

Tham khảo thêm: [leptosfmt trên GitHub](https://github.com/leptos-rs/leptosfmt)
