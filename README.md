# GoNhanh ⚡

[![CI](https://github.com/khaphanspace/gonhanh.org/actions/workflows/ci.yml/badge.svg)](https://github.com/khaphanspace/gonhanh.org/actions/workflows/ci.yml)
[![License: GPL-3.0](https://img.shields.io/badge/License-GPL--3.0-blue.svg)](LICENSE)

**Bộ gõ tiếng Việt thế hệ mới** — được thiết kế từ đầu cho kỷ nguyên Unicode.

---

## Tầm nhìn

> *"Đã đến lúc tiếng Việt có một bộ gõ được xây dựng đúng cách."*

Năm 2000, UniKey ra đời và trở thành chuẩn mực. Nhưng đó là thời của Windows XP, bảng mã TCVN3, và Internet dial-up.

**Hôm nay là 2025.** Unicode đã thắng. macOS và Windows đều hỗ trợ tiếng Việt native. Nhưng chúng ta vẫn đang dùng những bộ gõ được thiết kế cho một thời đại đã qua.

GoNhanh không phải là "một UniKey khác". Đây là **tái định nghĩa** cách gõ tiếng Việt:

- **Chỉ Unicode** — Không TCVN3, không VNI Windows, không CP 1258
- **Phonology-first** — Engine dựa trên ngữ âm học, không phải bảng tra cứu
- **Native-first** — SwiftUI cho macOS, WPF cho Windows
- **Rust core** — Memory-safe, blazing fast, maintainable

## Ba Không

| | Cam kết |
|:---:|---|
| 🚫 | **Không thu phí** — Miễn phí mãi mãi. Không freemium, không premium, không donation nag. |
| 🚫 | **Không quảng cáo** — Không banner, không popup, không "upgrade to pro". Trải nghiệm thuần khiết. |
| 🚫 | **Không theo dõi** — Không thu thập dữ liệu, không gửi thông tin, không cần internet. 100% offline. |

## Triết lý

### Phonology-first Engine

GoNhanh không dùng bảng tra cứu 89 ký tự như các bộ gõ truyền thống.

Thay vào đó, engine được xây dựng trên **ngữ âm học tiếng Việt**:

```
Âm tiết = [Phụ âm đầu] + [Âm đệm] + Nguyên âm chính + [Âm cuối] + Thanh điệu
```

- Phân loại nguyên âm: âm chính, âm đệm, bán nguyên âm
- Thuật toán đặt dấu theo quy tắc ngữ âm (không hardcode từng trường hợp)
- Hỗ trợ cả kiểu cũ (`hoà`) và kiểu mới (`hòa`)

> 📖 Xem chi tiết: [Hệ thống ngữ âm tiếng Việt](docs/vietnamese-language-system.md)

### Native Experience

Mỗi platform có UI riêng, không phải Qt hay Electron:

| Platform | UI Framework | Status |
|----------|--------------|--------|
| macOS | SwiftUI | ✅ Available |
| Windows | WPF/WinUI | 🚧 Planned |

### Rust Core

```
┌─────────────────────────────────┐
│     Platform UI (Swift/WPF)    │
└───────────────┬─────────────────┘
                │ FFI (C ABI)
┌───────────────▼─────────────────┐
│         Rust Core Engine        │
│  • Memory-safe, no crashes      │
│  • <1ms latency per keystroke   │
│  • ~3MB binary, ~25MB RAM       │
└─────────────────────────────────┘
```

## So sánh

|  | GoNhanh | OpenKey | UniKey | EVKey |
|---|:---:|:---:|:---:|:---:|
| **Năm ra đời** | 2025 | 2019 | 2000 | 2018 |
| **Miễn phí** | ✅ | ✅ | ✅ | ✅ |
| **Không quảng cáo** | ✅ | ✅ | ✅ | ✅ |
| **Open source** | ✅ | ✅ | ⚠️ | ✅ |
| **Chỉ Unicode** | ✅ | ❌ | ❌ | ❌ |
| **macOS native** | SwiftUI | Obj-C | Qt | Qt |
| **Engine** | Rust | C++ | C++ | C++ |

> GoNhanh không thay thế các bộ gõ trên. Đây là lựa chọn cho những ai muốn **đơn giản, hiện đại, và đúng chuẩn**.

## Cam kết phát triển

### Từ tác giả

> *"Tôi xây dựng GoNhanh vì tôi cần nó. Và tôi sẽ duy trì nó vì tôi dùng nó mỗi ngày."*

- **Long-term support** — Dự án sẽ được duy trì ít nhất 5 năm (2025-2030)
- **Semantic versioning** — Breaking changes chỉ ở major versions
- **Backward compatible** — Config và settings được bảo toàn qua các phiên bản
- **Community-driven** — Issues và PRs được review trong 48 giờ

### Roadmap

| Version | Target | Features |
|---------|--------|----------|
| 0.1 | Q1 2025 | macOS beta, Telex + VNI |
| 0.2 | Q2 2025 | Stable release, auto-update |
| 0.3 | Q3 2025 | Windows support |
| 1.0 | Q4 2025 | Production ready |

## Installation

### macOS (Build from source)

```bash
git clone https://github.com/khaphanspace/gonhanh.org
cd gonhanh.org
make build

# Install
cp -r platforms/macos/build/Release/GoNhanh.app /Applications/
```

### Homebrew (Coming soon)

```bash
brew install gonhanh
```

## Usage

1. Mở GoNhanh từ Applications
2. Cấp quyền Accessibility (System Settings → Privacy & Security)
3. Click icon menu bar để bật/tắt
4. Right-click để mở Settings

## Development

```bash
make test    # Run 99 tests
make build   # Build everything
make clean   # Clean artifacts
```

> 📖 [Development Guide](docs/development.md) · [Architecture](docs/architecture.md)

## Acknowledgments

Dự án được xây dựng trên vai những người khổng lồ:

- [UniKey](https://www.unikey.org/) — Bộ gõ huyền thoại, nguồn cảm hứng ban đầu
- [OpenKey](https://github.com/tuyenvm/OpenKey) — Tiên phong open source Vietnamese IME
- [EVKey](https://evkeyvn.com/) — Những cải tiến đáng giá cho cộng đồng

## Contributing

Contributions welcome! Xem [CONTRIBUTING.md](CONTRIBUTING.md)

## License

[GPL-3.0-or-later](LICENSE)

Tự do sử dụng, sửa đổi, phân phối — với điều kiện giữ nguyên license.

---

<p align="center">
  <i>Được xây dựng với ❤️ cho cộng đồng người Việt</i>
</p>
