#!/bin/bash
# Generate release notes using AI (opencode CLI)

LAST_RELEASE=$(gh release view --json tagName -q .tagName 2>/dev/null || echo "")
VERSION="$1"

if [ -n "$LAST_RELEASE" ]; then
    COMMITS=$(git log "$LAST_RELEASE"..HEAD --pretty=format:"%s" 2>/dev/null)
else
    COMMITS=$(git log --pretty=format:"%s" -20 2>/dev/null)
fi

if [ -z "$COMMITS" ]; then
    echo "No commits found"
    exit 1
fi

opencode run --format json "Tạo release notes cho version $VERSION của 'Gõ Nhanh' (Vietnamese IME for macOS).

Commits:
$COMMITS

Quy tắc:
- Nhóm theo: ✨ Tính năng mới, 🐛 Sửa lỗi, ⚡ Cải thiện, 🔧 Khác
- Bỏ qua section rỗng
- Mỗi item: 1 dòng, súc tích, viết tiếng Việt (có thể dùng keywords tiếng Anh như build, config, API...)
- Chỉ output markdown, không giải thích" 2>/dev/null | jq -r 'select(.type == "text") | .part.text'
