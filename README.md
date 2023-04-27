# LoL Account Manager 2
I got tired of having a `lolaccounts.txt` on my Desktop and manually copy and pasting passwords. This is a Rust rewrite of my [original account manager](https://github.com/sidit77/LoLAccountManager2).
Reasons for this rewrite:
* Better encryption
* Smaller binary
* Significantly inproved UI
  * Themes
  * More responsive
  * A search bar
  * Abillity to discard changes
  * Almost cross-platform
* 🦀

## Todo
- [ ] Remove custom fork of `age`
- [ ] Implement error handling
- [ ] Implement export options
- [x] Block closing until saving is complete

## Demo

https://user-images.githubusercontent.com/5053369/223086843-caba0799-02a4-4530-9dbd-67ad6e9269a2.mp4

## Security

The usernames and passwords are encrypted using [age](https://github.com/str4d/rage/tree/main/age). The password for this file is stored in the windows credential manager and never leaves the current pc. It's probably a good idea to keep in mind that this isn't meant to be a secure password manager but a more comfortable replacement for a plain text file. I have no experience writing secure software. Autofill is implemented by simulating key pressed: [Autofill helper](https://github.com/sidit77/LoLAccountManager2/blob/main/src/os/windows.rs).

## Download

[Github release page](https://github.com/sidit77/LoLAccountManager2/releases)

## Building from source

Install Rust
  * [Download](https://www.rust-lang.org/tools/install)

Clone the repository:
````powershell
git clone https://github.com/sidit77/LoLAccountManager2.git
cd LoLAccountManager
````

Compile:
````powershell
cargo build --release
````

The finished binaries are located in `.\target\release\`
