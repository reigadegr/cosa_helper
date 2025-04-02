cargo fmt
rm -rf output
rm -rf $(find ./target/aarch64-linux-android/debug -name "*cosa_helper*")

clear_crash() {
    rm -rf $(find ./target/aarch64-linux-android/debug -name "*mimalloc*")
    rm -rf $(find ./target/aarch64-linux-android/debug -name "*ndk*")
}

python3 ./make.py build --debug --nightly
