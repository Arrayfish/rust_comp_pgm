#!/usr/bin/env bash
# サンプルケースのテスト
#
# 使い方:
#   ./test.sh <コンテスト> <問題>   (リポジトリのルートで)  例: ./test.sh abc366 c
#   ../test.sh <問題>               (コンテストのディレクトリで) 例: ../test.sh c
#
# サンプルは <コンテスト>/samples/<問題>/ に N.in と N.out の組で置く。
#   例: abc366/samples/c/1.in, abc366/samples/c/1.out
# 出力の比較では、各行末の空白と末尾の空行を無視する。

set -u

if [ $# -eq 2 ]; then
    contest_dir=$1
    problem=$2
elif [ $# -eq 1 ]; then
    contest_dir=.
    problem=$1
else
    echo "使い方: $0 <コンテスト> <問題>  または  $0 <問題>" >&2
    exit 2
fi

if [ ! -f "$contest_dir/Cargo.toml" ]; then
    echo "エラー: $contest_dir/Cargo.toml がありません" >&2
    exit 2
fi
cd "$contest_dir" || exit 2

sample_dir="samples/$problem"
shopt -s nullglob
inputs=("$sample_dir"/*.in)
if [ ${#inputs[@]} -eq 0 ]; then
    echo "エラー: $sample_dir/*.in がありません" >&2
    exit 2
fi

if [ -t 1 ]; then
    green=$'\e[32m'; red=$'\e[31m'; yellow=$'\e[33m'; reset=$'\e[0m'
else
    green=; red=; yellow=; reset=
fi

# 先にビルドし、できた実行ファイルを直接実行する（cargo run の起動時間を計測に含めないため）
# コンパイルエラーならここで終了する
build_log=$(cargo build -q --bin "$problem" --message-format=json-render-diagnostics) || exit 1
exe=$(printf '%s\n' "$build_log" | sed -n 's/.*"executable":"\([^"]*\)".*/\1/p' | tail -n 1)
if [ -z "$exe" ] || [ ! -x "$exe" ]; then
    echo "エラー: 実行ファイルが見つかりません" >&2
    exit 1
fi

# 行末の空白と末尾の空行を取り除く（最終行の改行の有無も揃える）
normalize() {
    awk '{ sub(/[ \t\r]+$/, ""); if ($0 == "") { blank++ } else { while (blank > 0) { print ""; blank-- } print } }' "$1"
}

now_ms() {
    perl -MTime::HiRes=time -e 'printf "%d\n", time * 1000'
}

tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT

total=0
passed=0
for input in "${inputs[@]}"; do
    name=$(basename "$input" .in)
    expected="$sample_dir/$name.out"
    total=$((total + 1))

    start=$(now_ms)
    "$exe" < "$input" > "$tmp/actual" 2> "$tmp/stderr"
    status=$?
    elapsed=$(( $(now_ms) - start ))

    if [ $status -ne 0 ]; then
        echo "${red}RE${reset}  $name (${elapsed}ms, 終了コード $status)"
        cat "$tmp/stderr"
    elif [ ! -f "$expected" ]; then
        echo "${yellow}??${reset}  $name (${elapsed}ms, $expected がないので出力だけ表示)"
        cat "$tmp/actual"
    elif diff <(normalize "$expected") <(normalize "$tmp/actual") > "$tmp/diff"; then
        echo "${green}AC${reset}  $name (${elapsed}ms)"
        passed=$((passed + 1))
    else
        echo "${red}WA${reset}  $name (${elapsed}ms)"
        echo "  --- 期待する出力 / +++ 実際の出力"
        diff -u <(normalize "$expected") <(normalize "$tmp/actual") | tail -n +3 | sed 's/^/  /'
    fi
done

echo "結果: $passed / $total"
[ $passed -eq $total ]
