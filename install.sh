#!/bin/bash
set -e

# Расширяем крейт и генерируем C-заголовок
cargo expand --package objc_kvo_rust --lib > expanded.rs
cbindgen expanded.rs -l c > rust.h
rm expanded.rs

# Собираем универсальную статическую библиотеку для iOS
cargo lipo --release --targets aarch64-apple-ios

# Путь для копирования результатов (относительный к вашему рабочему каталогу)
DEST_PATH=~/Desktop/dev/objc_kvo_rust

mkdir -p $DEST_PATH/objc_kvo_rust/include/
cp rust.h $DEST_PATH/objc_kvo_rust/include/RustLibrary.h

mkdir -p $DEST_PATH/objc_kvo_rust/libs/
cp target/aarch64-apple-ios/release/libobjc_kvo_rust.a $DEST_PATH/objc_kvo_rust/libs/libobjc_kvo_rust.a

echo "Библиотека успешно собрана и скопирована в $DEST_PATH/objc_kvo_rust"
