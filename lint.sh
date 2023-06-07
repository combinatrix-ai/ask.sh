shfmt -i 4 -l -w install.sh
shellcheck install.sh
rustfmt --edition 2021 src/*
