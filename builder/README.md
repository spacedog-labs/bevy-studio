the builder needs to:

1. take auth to storage
2. take a target project in storage
3. take a build number
4. take a output folder
5. downloads the source
6. builds the source and records the stdout/stderr
7. uploads stdout in project build logs
8. uploads the release folder

buuld notes:

cargo build --target x86_64-unknown-linux-gnu

cargo build --target aarch64-unknown-linux-gnu

docker run --rm \
 --volume "${PWD}":/root/src \
 --workdir /root/src \
 joseluisq/rust-linux-darwin-builder:1.60.0 \
 sh -c "cargo build --target linux/arm64/v8"

If you need to have openssl@1.1 first in your PATH, run:
echo 'export CPPFLAGS="-I/opt/homebrew/opt/openssl@1.1/include"' >> ~/.zshrc

For compilers to find openssl@1.1 you may need to set:
export LDFLAGS="-L/opt/homebrew/opt/openssl@1.1/lib"
export CPPFLAGS="-I/opt/homebrew/opt/openssl@1.1/include"

For pkg-config to find openssl@1.1 you may need to set:
export PKG_CONFIG_PATH="/opt/homebrew/opt/openssl@1.1/lib/pkgconfig"

OPENSSL_DIR

docker run -it -v "$(pwd):/source" rust:buster cargo build --target=aarch64-unknown-linux-gnu

copy ./target/debug/builder /builder
RUN chmod +x ./builder
