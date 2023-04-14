# Songbird Android Native Library

[![](https://jitpack.io/v/tsirysndr/songbird-android.svg)](https://jitpack.io/#tsirysndr/songbird-android)

This is the Songbird Android Native Library.
See [Songbird](https://github.com/tsirysndr/music-player) for more information.

## Installation

This project requires [Rust](https://www.rust-lang.org/tools/install) and [Android NDK](https://developer.android.com/ndk/downloads) to be installed.

```sh
# Install dependencies
brew install protobuf # macOS
sudo apt-get install -y protobuf-compiler # Ubuntu/Debian
# Clone the repository
git clone https://github.com/tsirysndr/songbird-android-rs
cd songbird-android-rs
cargo install cargo-ndk
RUSTFLAGS="-C link-arg=-lc++_shared" cargo ndk -t armeabi-v7a -t arm64-v8a -o ./jniLibs/ build --release
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## License

This project is licensed under the terms of the MIT license. See the [LICENSE](LICENSE) file.