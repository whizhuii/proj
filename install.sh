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

    # 4. shell integration (rc files)
    local rc_files=()
    for f in "$HOME/.zshrc" "$HOME/.bashrc" "$HOME/.bash_profile" "$HOME/.zprofile"; do
        [[ -f "$f" ]] && rc_files+=("$f")
    done

    if [[ ${#rc_files[@]} -eq 0 ]]; then
        local shell_name
        shell_name="$(basename "${SHELL:-/bin/sh}")"
        case "${shell_name}" in
            zsh)  rc_files=("$HOME/.zshrc") ;;
            bash) rc_files=("$HOME/.bashrc") ;;
            *)    rc_files=("$HOME/.profile") ;;
        esac
        info "no existing rc file found, will create ${rc_files[0]}"
    fi

    local selected=()
    if [[ ${#rc_files[@]} -eq 1 ]]; then
        selected=("${rc_files[@]}")
    else
        local toggle=()
        for ((i=0; i<${#rc_files[@]}; i++)); do
            toggle+=(0)
        done

        while true; do
            printf "\n  Select rc files to update (Space to toggle, Enter to confirm):\n\n"
            for ((i=0; i<${#rc_files[@]}; i++)); do
                local mark=" "
                [[ ${toggle[$i]} -eq 1 ]] && mark="*"
                printf "  [%s] %d) %s\n" "$mark" $((i+1)) "${rc_files[$i]}"
            done
            printf "\n  Enter number to toggle, or press Enter to confirm: "
            read -r input
            [[ -z "$input" ]] && break

            if [[ "$input" =~ ^[0-9]+$ ]] && (( input >= 1 && input <= ${#rc_files[@]} )); then
                local idx=$((input-1))
                toggle[$idx]=$(( 1 - toggle[$idx] ))
            fi
        done

        for ((i=0; i<${#rc_files[@]}; i++)); do
            [[ ${toggle[$i]} -eq 1 ]] && selected+=("${rc_files[$i]}")
        done

        if [[ ${#selected[@]} -eq 0 ]]; then
            info "no file selected, defaulting to ${rc_files[0]}"
            selected=("${rc_files[0]}")
        fi
    fi

    for rc in "${selected[@]}"; do
        local shell_name
        case "$(basename "$rc")" in
            .zshrc|.zprofile)       shell_name="zsh" ;;
            .bashrc|.bash_profile)  shell_name="bash" ;;
            *)                      shell_name="$(basename "${SHELL:-/bin/sh}")" ;;
        esac

        cat >> "$rc" <<-RCEOF

# ---- proj project manager ----
export PATH="\${PATH}:${BINDIR}"
eval "\$(${BINARY} shell func)"
eval "\$(${BINARY} shell completion --mode ${shell_name})"
RCEOF
        donef "added proj to ${rc}"
    done

    if [[ ${#selected[@]} -gt 0 ]]; then
        echo ""
        info "Restart your shell or source the files above to apply changes."
    fi

    echo ""
    donef "proj is ready. Type: proj help"
    echo ""
}

main "$@"
