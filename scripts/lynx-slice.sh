#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 1 ]]; then
  echo "用法：$0 <number>"
  echo "例如：$0 674"
  exit 1
fi

NUMBER="$1"
if ! [[ "$NUMBER" =~ ^[0-9]+$ ]]; then
  echo "[ERROR] number 必須是整數：$NUMBER" >&2
  exit 1
fi

if ! command -v lynx >/dev/null 2>&1; then
  echo "[ERROR] 未找到 lynx，請先安裝：brew install lynx" >&2
  exit 1
fi

URL="https://algo.monster/liteproblems/${NUMBER}"
OUTDIR="./problem-src"
OUTFILE="${OUTDIR}/${NUMBER}.md"

mkdir -p "$OUTDIR"

# 抓取並切片：起點=行首為 NUMBER 的第一行；終點=行首為 Intuition 的行（不包含）
lynx -dump -nolist -nonumbers -assume_charset=utf-8 -display_charset=utf-8 "$URL" \
| awk -v num="$NUMBER" '
    start==0 && $0 ~ "^[[:space:]]*" num { start=1 }
    start {
      if ($0 ~ "^[[:space:]]*Quick Interview Experience") exit
      print
    }
  ' > "$OUTFILE"

echo "已儲存：$OUTFILE"
