#!/usr/bin/env bash
set -uo pipefail

REPO="codeignus/awesome-skills-cli"
INSTALL_DIR="${HOME}/.local/bin"

err() { echo "Error: $*" >&2; exit 1; }

detect_arch() {
    local os arch
    os="$(uname -s)"
    arch="$(uname -m)"
    case "$os" in
        Linux)   os="linux" ;;
        Darwin)  os="macos" ;;
        *)       echo "Unsupported OS: $os" >&2; exit 1 ;;
    esac
    case "$arch" in
        x86_64|amd64) arch="x64" ;;
        aarch64|arm64) arch="arm64" ;;
        *)             echo "Unsupported architecture: $arch" >&2; exit 1 ;;
    esac
    echo "awesome-skills-cli-${os}-${arch}"
}

main() {
    local artifact

    artifact="$(detect_arch)"
    platform="${artifact#awesome-skills-cli-}"

    echo "Detected platform: ${platform}"
    echo "Fetching latest release info..."
    latest_url="https://api.github.com/repos/${REPO}/releases/latest"
    api_response=$(curl -sL "$latest_url") || err "Failed to reach GitHub API (network error or rate limited)"
    download_url=$(echo "$api_response" | grep -o "\"browser_download_url\": *\"[^\"]*${artifact}\"" | head -1 | sed 's/.*"browser_download_url": *"//' | sed 's/"$//')

    if [ -z "$download_url" ]; then
        echo "Error: no binary found for ${artifact}" >&2
        exit 1
    fi

    echo "Latest release found: ${download_url##*/}"

    checksum_file="${download_url%/*}/checksums.txt"

    tmp_dir="$(mktemp -d)"
    trap 'rm -rf "${tmp_dir:-}"' EXIT

    echo "Downloading ${artifact}..."
    curl -fL --progress-bar "$download_url" -o "${tmp_dir}/${artifact}" || err "Download failed for ${artifact}"

    echo "Downloading checksums..."
    curl -fL --progress-bar "$checksum_file" -o "${tmp_dir}/checksums.txt" || err "Download failed for checksums"

    echo "Verifying checksum..."
    grep "${artifact}" "${tmp_dir}/checksums.txt" > "${tmp_dir}/${artifact}.sha256"
    (cd "$tmp_dir" && sha256sum --check "${artifact}.sha256" 2>/dev/null || shasum -a 256 --check "${artifact}.sha256") || err "Checksum verification failed"

    mkdir -p "$INSTALL_DIR"
    chmod +x "${tmp_dir}/${artifact}"
    mv "${tmp_dir}/${artifact}" "${INSTALL_DIR}/awesome-skills-cli"
    echo "Installed ${INSTALL_DIR}/awesome-skills-cli"

    if ! command -v awesome-skills-cli &>/dev/null; then
        echo "Note: ${INSTALL_DIR} is not in your PATH. Add it with:"
        echo "  export PATH=\"${INSTALL_DIR}:\$PATH\""
    fi
}

main
