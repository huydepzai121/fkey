# Hướng dẫn cài đặt GoNhanh trên macOS

## Yêu cầu hệ thống

- macOS 13.0 (Ventura) trở lên.
- Chip Apple Silicon (M1/M2/M3) hoặc Intel.

## Các bước cài đặt

### 1. Tải ứng dụng

Truy cập trang [Releases](https://github.com/khaphanspace/gonhanh.org/releases) và tải xuống file `.dmg` phiên bản mới nhất (ví dụ: `GoNhanh-v1.0.0.dmg`).

### 2. Cài đặt

1. Mở file `.dmg` vừa tải.
2. Kéo biểu tượng **GoNhanh** vào thư mục **Applications**.

### 3. Cấp quyền truy cập

Để bộ gõ hoạt động, bạn cần cấp quyền **Accessibility** (Trợ năng) để ứng dụng có thể lắng nghe phím bấm:

1. Mở **GoNhanh** từ Applications.
2. Hệ thống sẽ yêu cầu quyền truy cập. Nhấn **Open System Settings**.
3. Tìm **GoNhanh** trong danh sách và bật công tắc.
4. Nếu ứng dụng không tự khởi động lại, hãy mở lại thủ công.

### 4. Thiết lập lần đầu

- Biểu tượng GoNhanh sẽ xuất hiện trên thanh Menu Bar (góc trên bên phải).
- Mặc định bộ gõ sẽ ở chế độ **Telex** và tự động khởi động cùng máy.

## Gỡ cài đặt

Để xóa hoàn toàn GoNhanh:

1. Thoát ứng dụng (Click icon trên Menu Bar -> Quit).
2. Xóa GoNhanh khỏi thư mục Applications.
3. (Tùy chọn) Xóa file cấu hình tại `~/.config/gonhanh`.
