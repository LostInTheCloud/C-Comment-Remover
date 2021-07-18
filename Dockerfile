from archlinux:latest
# copy . /root/codeonly
run pacman -Syu fish cargo rustup gcc --noconfirm
run chsh -s /bin/fish
run rustup install nightly