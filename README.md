# csbindgen-handson
csbindgenのハンズオン用リポジトリ

## やり方
[プレゼン資料](https://aiueo-1234.github.io/csbindgen-handson/)をもとに進めていくと、文字操作を除いてcsbindgenを用いた簡単なFFIを試すことができます。  
また、ほぼプレゼン資料通りにコミットを重ねた[kc3ブランチ](https://github.com/aiueo-1234/csbindgen-handson/tree/kc3)があるのでこれを参考にしながら行うこともできます。

## 環境
- Windows 10 / 11 (armを除く)　または Ubuntu / WSL2
- .NET 8
- Rust
- LLVM(Clang)
  - bindgenにて必要。詳細は[ここを確認](https://rust-lang.github.io/rust-bindgen/requirements.html)

## 使用ライブラリ
- csbindgen
  - https://github.com/Cysharp/csbindgen/
- bindgen
  - https://github.com/rust-lang/rust-bindgen
- cc
  - https://github.com/rust-lang/cc-rs