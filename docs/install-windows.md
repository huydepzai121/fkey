# Hướng dẫn cài đặt GoNhanh trên Windows

> **Lưu ý:** Phiên bản Windows hiện đang trong giai đoạn phát triển (Planned).

## Trạng thái hiện tại

GoNhanh cho Windows đang được phát triển sử dụng **WPF (Windows Presentation Foundation)** kết hợp với lõi **Rust**.

## Dành cho Developers

Nếu bạn muốn đóng góp hoặc thử nghiệm phiên bản dev:

### Yêu cầu

- Windows 10/11
- Rust toolchain
- Visual Studio (C++ workload)

### Build từ source

```bash
git clone https://github.com/khaphanspace/gonhanh.org.git
cd gonhanh.org/platforms/windows
cargo build --release
```

Hãy theo dõi [Roadmap](../README.md) để cập nhật thông tin mới nhất.
