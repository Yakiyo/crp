## update and install some things we should probably have
apt-get update
apt-get install -y \
  curl \
  git \
  gnupg2 \
  jq \
  sudo \
  zsh \
  bash \
  vim \
  build-essential \
  openssl \
  unzip

## Install rustup and common components
curl https://sh.rustup.rs -sSf | sh -s -- -y 

rustup component add rustfmt
rustup component add clippy 

curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

echo alias bat="bat --pager=never" >> ~/.bashrc
# Install oh-my-posh
curl -s https://ohmyposh.dev/install.sh | bash -s

echo eval "$(oh-my-posh init bash)" >> ~/.bashrc
exec bash