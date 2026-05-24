#!/usr/bin/env bash
set -euo pipefail

REPO="whizhuii/proj"
BINARY="proj-core"
BINDIR="${HOME}/.local/bin"

# ---- helpers ----
info()  { printf "  \033[1m·\033[0m %s\n" "$*"; }
warn()  { printf "  \033[33m!\033[0m %s\n" "$*" >&2; }
err()   { printf "  \033[31mx\033[0m %s\n" "$*" >&2; exit 1; }
donef() { printf "  \033[32m✓\033[0m %s\n" "$*"; }

detect_platform() {
    local os arch
    case "$(uname -s)" in
        Linux)  os="unknown-linux-gnu" ;;
        Darwin) os="apple-darwin" ;;
        *)      err "unsupported OS: $(uname -s)" ;;
    esac
    case "$(uname -m)" in
        x86_64|amd64) arch="x86_64" ;;
        aarch64|arm64) arch="aarch64" ;;
        *)            err "unsupported arch: $(uname -m)" ;;
    esac
    echo "${arch}-${os}"
}

# ---- main ----
main() {
    echo ""
    info "proj — project directory management"
    echo ""

    # 1. resolve download URL
    local target
    target="$(detect_platform)"
    local archive="proj-core-${target}.tar.gz"
    local url="https://github.com/${REPO}/releases/latest/download/${archive}"

    # 2. download & extract
    info "downloading ${archive} …"
    local tmpdir
    tmpdir="$(mktemp -d)"
    pushd "${tmpdir}" >/dev/null

    if command -v curl >/dev/null 2>&1; then
        curl -fsSLO "${url}" || err "download failed"
    elif command -v wget >/dev/null 2>&1; then
        wget -q "${url}" || err "download failed"
    else
        err "need curl or wget"
    fi

    tar xzf "${archive}" || err "extract failed"

    # 3. install binary
    mkdir -p "${BINDIR}"
    install -m 755 "${BINARY}" "${BINDIR}/${BINARY}"
    popd >/dev/null
    rm -rf "${tmpdir}"
    donef "installed ${BINARY} to ${BINDIR}"

    # 4. check PATH
    if ! echo "${PATH}" | tr ':' '\n' | grep -qFx "${BINDIR}"; then
        warn "${BINDIR} is not in PATH — add it to your rc file:"
        echo "    export PATH=\"\${PATH}:${BINDIR}\""
    fi

    # 5. shell integration
    local rc=""
    local shell_name
    shell_name="$(basename "${SHELL:-/bin/sh}")"
    case "${shell_name}" in
        zsh) rc="${ZDOTDIR:-${HOME}}/.zshrc" ;;
        bash) rc="${HOME}/.bashrc" ;;
        *)    warn "unknown shell '${shell_name}' — add eval manually (see README)" ;;
    esac

    if [[ -n "${rc}" ]] && [[ -f "${rc}" ]]; then
        if grep -q "proj-core shell" "${rc}" 2>/dev/null; then
            donef "shell integration already set up in ${rc}"
        else
            cat >> "${rc}" <<-RCEOF

# ---- proj project manager ----
eval "\$(${BINARY} shell func)"
eval "\$(${BINARY} shell completion --shell ${shell_name})"
RCEOF
            donef "added shell integration to ${rc}"
            info "restart shell or run: source ${rc}"
        fi
    fi

    echo ""
    donef "proj is ready. Type: proj help"
    echo ""
}

main "$@"
