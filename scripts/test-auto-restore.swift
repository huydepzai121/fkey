#!/usr/bin/env swift
// Auto-restore test for GoNhanh - Tests English word detection
// Tests common English words that macOS Telex incorrectly converts

import Foundation
import CoreGraphics

let keycodes: [Character: UInt16] = [
    "a": 0, "s": 1, "d": 2, "f": 3, "h": 4, "g": 5, "z": 6, "x": 7, "c": 8, "v": 9,
    "b": 11, "q": 12, "w": 13, "e": 14, "r": 15, "y": 16, "t": 17, "1": 18, "2": 19,
    "3": 20, "4": 21, "6": 22, "5": 23, "9": 25, "7": 26, "8": 28, "0": 29,
    "o": 31, "u": 32, "i": 34, "p": 35, "l": 37, "j": 38, "k": 40, "n": 45, "m": 46,
    " ": 49, ",": 43, ".": 47, "[": 33, "]": 30, ":": 41, "/": 44
]

let configPath = "/tmp/gonhanh_config.txt"

var typeDelay: UInt32 = 50000  // 50ms between keys

func typeKey(_ char: Character) {
    guard let keycode = keycodes[char] else { return }
    guard let source = CGEventSource(stateID: .combinedSessionState) else { return }
    if let down = CGEvent(keyboardEventSource: source, virtualKey: keycode, keyDown: true),
       let up = CGEvent(keyboardEventSource: source, virtualKey: keycode, keyDown: false) {
        down.post(tap: .cghidEventTap)
        usleep(5000)
        up.post(tap: .cghidEventTap)
        usleep(typeDelay)
    }
}

func typeString(_ str: String) {
    for char in str.lowercased() {
        typeKey(char)
    }
}

func setConfig(_ config: String) {
    try? config.write(toFile: configPath, atomically: true, encoding: .utf8)
    usleep(50000)
}

// ═══════════════════════════════════════════════════════════════════════════════
// AUTO-RESTORE TEST - Common English words that macOS Telex breaks
// ═══════════════════════════════════════════════════════════════════════════════
//
// | Gõ      | macOS Telex | Gõ Nhanh |
// |---------|-------------|----------|
// | text    | têt ❌      | text ✅  |
// | expect  | ễpct ❌     | expect ✅|
// | perfect | pềct ❌     | perfect ✅|
// | window  | ưindow ❌   | window ✅|
// | with    | ưith ❌     | with ✅  |
// | tesla   | téla ❌     | tesla ✅ |
// | luxury  | lủuy ❌     | luxury ✅|
// | their   | thỉ ❌      | their ✅ |
// | wow     | ươ ❌       | wow ✅   |
// | file    | file ✅     | file ✅  |
// | fix     | fix ✅      | fix ✅   |
//
// ═══════════════════════════════════════════════════════════════════════════════

// Short paragraph mixing English and Vietnamese for auto-restore testing
// Telex input: Vietnamese words use telex (dduwowcj = được, tieengs = tiếng, etc.)
let testParagraph = "Tooi ddang test auto restore vowis cacs tuwd tieengs Anh nhuw text, expect, perfect, window, with, their, luxury, tesla, wow, file, fix. Neeus hoatj ddoongj ddungs thif seex khoonng bij loix. I expect this text file to be perfect. Mowrf window leenf xem their luxury tesla, wow thatj ddepj. Please fix this issue with the system. Tooi thaays nos hoatj ddoongj raats toots."

// Expected output: Vietnamese converted, English preserved
let expectedOutput = "Tôi đang test auto restore với các từ tiếng Anh như text, expect, perfect, window, with, their, luxury, tesla, wow, file, fix. Nếu hoạt động đúng thì sẽ không bị lỗi. I expect this text file to be perfect. Mở window lên xem their luxury tesla, wow thật đẹp. Please fix this issue with the system. Tôi thấy nó hoạt động rất tốt."

func runTest() {
    print("")
    print(" Auto-Restore Test")
    print(" Click vào input field ngay!")
    print("")
    print(" 3..."); sleep(1)
    print(" 2..."); sleep(1)
    print(" 1..."); sleep(1)

    setConfig("electron,8000,15000,8000")

    print(" Đang gõ (delay: \(typeDelay/1000)ms)...")

    for char in testParagraph.lowercased() {
        typeKey(char)
    }

    print("")
    print(" Xong!")
    print("")
    print(" Expected: \(expectedOutput)")
    print("")
}

// Main
print("")
print("══════════════════════════════════════════")
print("     GoNhanh Auto-Restore Test")
print("══════════════════════════════════════════")
print("")
print(" Input (Telex):")
print(" \"\(testParagraph)\"")
print("")
print(" Expected:")
print(" \"\(expectedOutput)\"")
print("")
print(" Press Enter to start...")
_ = readLine()

runTest()
