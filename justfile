# Targets that open the Bluetooth management socket and need CAP_NET_ADMIN
examples := "monitor scan"
binaries := ""

profile_dir := "target/release"

default: release

# Build release binaries and examples, then grant CAP_NET_ADMIN
release: build setcap

# Build release binaries and examples
build:
    cargo build --release --bins --examples

# Grant CAP_NET_ADMIN to the release targets, asking for sudo
setcap:
    #!/usr/bin/env bash
    set -euo pipefail
    targets=()
    for name in {{ binaries }}; do targets+=("{{ profile_dir }}/$name"); done
    for name in {{ examples }}; do targets+=("{{ profile_dir }}/examples/$name"); done
    pending=()
    for target in "${targets[@]}"; do
        if [[ ! -x "$target" ]]; then
            echo "missing $target, run 'just build' first" >&2
            exit 1
        fi
        if getcap "$target" | grep -q cap_net_admin; then
            echo "$target already has cap_net_admin"
        else
            pending+=("$target")
        fi
    done
    if [[ ${#pending[@]} -eq 0 ]]; then
        exit 0
    fi
    echo "Setting cap_net_admin+ep requires sudo for: ${pending[*]}"
    if [[ ! -t 0 ]]; then
        echo "Not running interactively, skipping setcap." >&2
        exit 0
    fi
    read -r -p "Use sudo to set capabilities? [y/N] " answer
    if [[ ! "$answer" =~ ^[Yy]([Ee][Ss])?$ ]]; then
        echo "Skipping setcap. Run the targets as root instead."
        exit 0
    fi
    if ! sudo -v; then
        echo "No sudo access, skipping setcap. Run the targets as root instead." >&2
        exit 0
    fi
    for target in "${pending[@]}"; do
        sudo setcap cap_net_admin+ep "$target"
        echo "$target: $(getcap "$target")"
    done
