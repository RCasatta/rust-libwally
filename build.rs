extern crate cc;

use std::process::Command;

fn main() {

        Command::new("./tools/cleanup.sh")
            .current_dir("./depend/libwally-core/")
            .status()
            .expect("failed to execute cleanup");

        Command::new("./tools/autogen.sh")
            .current_dir("./depend/libwally-core/")
            .status()
            .expect("failed to execute autogen");

        Command::new("./configure")
            .arg("--enable-elements")
            .current_dir("./depend/libwally-core/")
            .status()
            .expect("failed to execute configure");

        cc::Build::new()
            .include("depend/libwally-core/")
            .include("depend/libwally-core/src")
            .include("depend/libwally-core/include")
            .file("depend/libwally-core/src/aes.c")
            .file("depend/libwally-core/src/base58.c")
            .file("depend/libwally-core/src/bech32.c")
            .file("depend/libwally-core/src/bip32.c")
            .file("depend/libwally-core/src/bip38.c")
            .file("depend/libwally-core/src/bip39.c")
            .file("depend/libwally-core/src/elements.c")
            .file("depend/libwally-core/src/hex.c")
            .file("depend/libwally-core/src/hmac.c")
            .file("depend/libwally-core/src/internal.c")
            .file("depend/libwally-core/src/mnemonic.c")
            //.file("depend/libwally-core/src/pbkdf2.c")
            .file("depend/libwally-core/src/script.c")
            .file("depend/libwally-core/src/scrypt.c")
            .file("depend/libwally-core/src/sign.c")
            .file("depend/libwally-core/src/transaction.c")
            .file("depend/libwally-core/src/wif.c")
            .file("depend/libwally-core/src/wordlist.c")
            .debug(true)
            .compile("libwally.a");
}
