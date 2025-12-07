# Vietnamese Input Methods

## Vietnamese Alphabet

### Vowels (Nguyên âm) - 12 letters

| Base | With Circumflex (^) | With Horn (móc) | With Breve (˘) |
|------|---------------------|-----------------|----------------|
| a    | â                   | -               | ă              |
| e    | ê                   | -               | -              |
| i    | -                   | -               | -              |
| o    | ô                   | ơ               | -              |
| u    | -                   | ư               | -              |
| y    | -                   | -               | -              |

**All 12 vowels:** a, ă, â, e, ê, i, o, ô, ơ, u, ư, y

### Tone Marks (Dấu thanh) - 5 marks

| Name   | Vietnamese | Symbol | Example |
|--------|------------|--------|---------|
| Sắc    | Acute      | ´      | á       |
| Huyền  | Grave      | `      | à       |
| Hỏi    | Hook above | ̉       | ả       |
| Ngã    | Tilde      | ~      | ã       |
| Nặng   | Dot below  | .      | ạ       |

### Consonants (Phụ âm) - 17 letters

**Single:** b, c, d, đ, g, h, k, l, m, n, p, q, r, s, t, v, x

**Digraphs:** ch, gh, gi, kh, ng, nh, ph, qu, th, tr

**Trigraph:** ngh

---

## VNI Input Method

Uses number keys 1-9 to add diacritics.

### Tone Marks (1-5)

| Key | Mark   | Vietnamese | Result |
|-----|--------|------------|--------|
| 1   | Sắc    | Acute      | á é í ó ú ý |
| 2   | Huyền  | Grave      | à è ì ò ù ỳ |
| 3   | Hỏi    | Hook       | ả ẻ ỉ ỏ ủ ỷ |
| 4   | Ngã    | Tilde      | ã ẽ ĩ õ ũ ỹ |
| 5   | Nặng   | Dot below  | ạ ẹ ị ọ ụ ỵ |

### Vowel Modifiers (6-8)

| Key | Modifier   | Applies to | Result |
|-----|------------|------------|--------|
| 6   | Circumflex | a, e, o    | â, ê, ô |
| 7   | Horn       | o, u       | ơ, ư   |
| 8   | Breve      | a          | ă      |

### Special (9)

| Key | Result |
|-----|--------|
| 9   | đ (from d) |

### Examples

| Input    | Output | Explanation |
|----------|--------|-------------|
| a1       | á      | a + sắc |
| a2       | à      | a + huyền |
| a6       | â      | a + circumflex |
| a61      | ấ      | â + sắc |
| o7       | ơ      | o + horn |
| o72      | ờ      | ơ + huyền |
| u7       | ư      | u + horn |
| u72      | ừ      | ư + huyền |
| a8       | ă      | a + breve |
| a81      | ắ      | ă + sắc |
| d9       | đ      | d + stroke |
| tu72     | từ     | t + ư + huyền |
| Vie65t   | Việt   | V + iệ + t |

### Double-key Revert

Pressing the same key twice reverts the change:
- `a11` → `a1` (not áá)
- `a66` → `a6` (not ââ)

---

## Telex Input Method

Uses letter keys to add diacritics.

### Tone Marks

| Key | Mark   | Vietnamese | Result |
|-----|--------|------------|--------|
| s   | Sắc    | Acute      | á é í ó ú ý |
| f   | Huyền  | Grave      | à è ì ò ù ỳ |
| r   | Hỏi    | Hook       | ả ẻ ỉ ỏ ủ ỷ |
| x   | Ngã    | Tilde      | ã ẽ ĩ õ ũ ỹ |
| j   | Nặng   | Dot below  | ạ ẹ ị ọ ụ ỵ |

### Vowel Modifiers

| Key | Modifier   | Applies to | Result |
|-----|------------|------------|--------|
| aa  | Circumflex | a          | â      |
| ee  | Circumflex | e          | ê      |
| oo  | Circumflex | o          | ô      |
| aw  | Breve      | a          | ă      |
| ow  | Horn       | o          | ơ      |
| uw  | Horn       | u          | ư      |

### Special

| Key | Result |
|-----|--------|
| dd  | đ (from d) |

### Examples

| Input      | Output  | Explanation |
|------------|---------|-------------|
| as         | á       | a + sắc |
| af         | à       | a + huyền |
| aa         | â       | a + circumflex |
| aas        | ấ       | â + sắc |
| ow         | ơ       | o + horn |
| owf        | ờ       | ơ + huyền |
| uw         | ư       | u + horn |
| uwf        | ừ       | ư + huyền |
| aw         | ă       | a + breve |
| aws        | ắ       | ă + sắc |
| dd         | đ       | d + stroke |
| tuwf       | từ      | t + ừ |
| Vieetj     | Việt    | V + iệ + t |

### Double-key Revert

Pressing the same key twice reverts the change:
- `ass` → `as` (not áá)
- `aaa` → `aa` (not ââ)
- `aww` → `aw` (not ăă)

### Order Flexibility

Telex allows flexible order for tone + mark:
- `owf` = `ofw` = ờ
- `aas` = `asa` = ấ (mark can come before or after tone key)

---

## Complete Vowel Matrix

All possible Vietnamese vowels with all tone marks:

### Base Vowels (a, e, i, o, u, y)

| Base | Sắc | Huyền | Hỏi | Ngã | Nặng |
|------|-----|-------|-----|-----|------|
| a    | á   | à     | ả   | ã   | ạ    |
| e    | é   | è     | ẻ   | ẽ   | ẹ    |
| i    | í   | ì     | ỉ   | ĩ   | ị    |
| o    | ó   | ò     | ỏ   | õ   | ọ    |
| u    | ú   | ù     | ủ   | ũ   | ụ    |
| y    | ý   | ỳ     | ỷ   | ỹ   | ỵ    |

### Circumflex Vowels (â, ê, ô)

| Base | Sắc | Huyền | Hỏi | Ngã | Nặng |
|------|-----|-------|-----|-----|------|
| â    | ấ   | ầ     | ẩ   | ẫ   | ậ    |
| ê    | ế   | ề     | ể   | ễ   | ệ    |
| ô    | ố   | ồ     | ổ   | ỗ   | ộ    |

### Horn Vowels (ơ, ư)

| Base | Sắc | Huyền | Hỏi | Ngã | Nặng |
|------|-----|-------|-----|-----|------|
| ơ    | ớ   | ờ     | ở   | ỡ   | ợ    |
| ư    | ứ   | ừ     | ử   | ữ   | ự    |

### Breve Vowel (ă)

| Base | Sắc | Huyền | Hỏi | Ngã | Nặng |
|------|-----|-------|-----|-----|------|
| ă    | ắ   | ằ     | ẳ   | ẵ   | ặ    |

---

## References

- [VNI - Wikipedia](https://en.wikipedia.org/wiki/VNI)
- [Telex (input method) - Wikipedia](https://en.wikipedia.org/wiki/Telex_(input_method))
- [Vietnamese language and computers - Wikipedia](https://en.wikipedia.org/wiki/Vietnamese_language_and_computers)
