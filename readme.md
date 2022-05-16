# mojimoji-rs
Rust implementation of a fast converter between Japanese hankaku and zenkaku characters, [mojimoji](https://github.com/studio-ousia/mojimoji).

## Installation
In `Cargo.toml`,
```
[dependencies]  
mojimoji-rs = "*"
```

## Zenkaku to Hankaku
### Definition
```rust
pub fn zen_to_han(text: String, ascii: bool, digit: bool, kana: bool) -> String
```
### Arguments
* `text` - text to convert.
* `ascii` - indicates whether to convert ascii characters.
* `digit` - indicates whether to convert digits.
* `kana` - indicates whether to convert Japanese characters.
### Examples
```rust
use mojimoji_rs::zen_to_han;
assert_eq!(zen_to_han("アイウａｂｃ０１２".to_string(), true, true, true), "ｱｲｳabc012".to_string());
assert_eq!(zen_to_han("アイウａｂｃ０１２".to_string(), true, true, false), "アイウabc012".to_string());
assert_eq!(zen_to_han("アイウａｂｃ０１２".to_string(), true, false, true), "ｱｲｳabc０１２".to_string());
assert_eq!(zen_to_han("アイウａｂｃ０１２".to_string(), false, true, true), "ｱｲｳａｂｃ012".to_string());
```
## Hankaku to Zenkaku

### Definition
```rust
pub fn han_to_zen(text: String, ascii: bool, digit: bool, kana: bool) -> String
```
### Arguments
* `text` - text to convert.
* `ascii` - indicates whether to convert ascii characters.
* `digit` - indicates whether to convert digits.
* `kana` - indicates whether to convert Japanese characters.
### Examples
```rust
use mojimoji_rs::zen_to_han;
assert_eq!(han_to_zen("ｱｲｳabc012".to_string(), true, true, true), "アイウａｂｃ０１２".to_string());
assert_eq!(han_to_zen("ｱｲｳabc012".to_string(), true, true, false), "ｱｲｳａｂｃ０１２".to_string());
assert_eq!(han_to_zen("ｱｲｳabc012".to_string(), true, false, true), "アイウａｂｃ012".to_string());
assert_eq!(han_to_zen("ｱｲｳabc012".to_string(), false, true, true), "アイウabc０１２".to_string());
```


