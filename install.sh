#!/usr/bin/env bash
set -euo pipefail

REPO="whizhuii/proj"
BINARY="proj-core"
BINDIR="${HOME}/.local/bin"

# ---- helpers ----
donef() { printf "  \033[32m✓\033[0m  %s\n" "$*"; }
hint()  { printf "      \033[2m↳ %s\033[0m\n" "$*"; }
err()   { printf "  \033[31mx\033[0m  %s\n" "$*" >&2; exit 1; }

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

write_config() {
    local use_fzf="$1"
    local config_dir="${HOME}/.config/proj"
    local config_file="${config_dir}/config.yaml"

    mkdir -p "${config_dir}"
    cat > "${config_file}" <<-CONF
rm_to: removed
init_to: develop
clone_to: uncategorized
sync_new_to: uncategorized
sync_missing_to: removed
visible_categories:
- develop
- stable
- uncategorized
use_fzf: ${use_fzf}
no_git: false
project_dir: ~/Project
CONF

    if [[ "${use_fzf}" == "true" ]]; then
        donef "Fzf mode configured"
    else
        donef "Pass mode configured (fzf not found)"
        hint "install fzf later for interactive picker"
    fi
}

configure_mode() {
    local config_file="${HOME}/.config/proj/config.yaml"

    if [[ -f "${config_file}" ]]; then
        donef "Config already exists at ${config_file}"
        hint "delete it to re-run mode setup"
        return
    fi

    if command -v fzf >/dev/null 2>&1; then
        printf "\n  fzf detected. Use interactive picker mode (recommended)? [Y/n] "
        read -r input
        case "${input:-Y}" in
            y|Y|yes|Yes|"")
                write_config "true"
                ;;
            *)
                write_config "false"
                hint "switch later by setting use_fzf: true in config"
                ;;
        esac
    else
        write_config "false"
    fi
}

# ---- main ----
main() {
    echo ""

    # 1. resolve download URL
    local target
    target="$(detect_platform)"
    local archive="proj-core-${target}.tar.gz"
    local url="https://github.com/${REPO}/releases/latest/download/${archive}"

    # 2. download & extract
    local tmpdir
    tmpdir="$(mktemp -d)"
    pushd "${tmpdir}" >/dev/null

    if command -v curl >/dev/null 2>&1; then
        curl -fsSLO "${url}" 2>/dev/null || err "download failed"
    elif command -v wget >/dev/null 2>&1; then
        wget -q "${url}" 2>/dev/null || err "download failed"
    else
        err "need curl or wget"
    fi

    tar xzf "${archive}" || err "extract failed"

    # 3. install binary
    mkdir -p "${BINDIR}"
    install -m 755 "${BINARY}" "${BINDIR}/${BINARY}"
    popd >/dev/null
    rm -rf "${tmpdir}"
    donef "proj-core installed to ${BINDIR}"

    # 4. configure mode (fzf or pass)
    configure_mode

    # 5. shell integration (rc files)
    local rc_files=()
    for f in "$HOME/.zshrc" "$HOME/.bashrc" "$HOME/.bash_profile" "$HOME/.zprofile"; do
        [[ -f "$f" ]] && rc_files+=("$f")
    done

    if [[ ${#rc_files[@]} -eq 0 ]]; then
        printf "  \033[33m!\033[0m  no shell rc file found (~/.zshrc, ~/.bashrc, etc)\n"
        hint "add manually: eval \"\$(proj-core shell func)\""
    else
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
        done

        local rc_name
        rc_name="$(basename "${selected[0]}")"
        if [[ ${#selected[@]} -gt 1 ]]; then
            rc_name="${rc_name} + $(( ${#selected[@]} - 1 ))"
        fi
        donef "Added proj to ${rc_name}"
    fi

    echo ""
    donef "proj is ready — restart your shell"
    echo ""
}

main "$@"
