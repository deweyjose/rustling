#!/bin/sh

RUSTLING_RELEASES="${RUSTLING_RELEASES:-https://github.com/deweyjose/rustling/releases}"
VERSION="${VERSION:-0.2.0}"

main() {
  need_cmd mkdir

  get_architecture || return 1

  local _arch="$RETVAL"

  local _file="rustling-${VERSION}-${_arch}-amd64"
  local _url="${RUSTLING_RELEASES}/download/${VERSION}/${_file}"

  mkdir -p rustling
  cd rustling

  ensure downloader "$_url" "$_file"

  local _file="patterns.json"
  local _url="${RUSTLING_RELEASES}/download/${VERSION}/${_file}"
  ensure downloader "$_url" "$_file"
}

downloader() {
  local _dld
  if check_cmd curl; then
    _dld=curl
  elif check_cmd wget; then
    _dld=wget
  else
    err "need curl or wget"
  fi

  if [ "$_dld" = curl ]; then
    _err=$(curl --proto '=https' --tlsv1.2 --silent --show-error --fail --location "$1" --output "$2" 2>&1)
    _status=$?
  else
    _err=$(wget "$1" -O "$2" 2>&1)
    _status=$?
  fi

  if [ -n "$_err" ]; then
    echo "$_err" >&2
    if echo "$_err" | grep -q ' 404 Not Found$'; then
      err "installer for platform '$3' not found, this may be unsupported"
    fi
  fi

  return $_status
}

say() {
  printf 'rustup: %s\n' "$1"
}

err() {
  say "$1" >&2
  exit 1
}

need_cmd() {
  if ! check_cmd "$1"; then
    err "need '$1' (command not found)"
  fi
}

check_cmd() {
  command -v "$1" >/dev/null 2>&1
}

ensure() {
  if ! "$@"; then err "command failed: $*"; fi
}

get_architecture() {
  # very simple for now - macos or linux
  local _ostype
  _ostype="$(uname -s)"

  case "$_ostype" in

  Linux)
    check_proc
    _ostype=linux
    _bitness=$(get_bitness)
    ;;

  Darwin)
    _ostype=macos
    ;;

  *)
    err "unrecognized OS type: $_ostype"
    ;;

  esac

  _arch="${_ostype}"

  RETVAL="$_arch"
}

main "$@" || exit 1
