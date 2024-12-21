# ベースイメージを作成します
FROM scratch

# spin.tomlをコピーします
COPY spin.toml /spin.toml

# Wasmファイルを、spin.tomlと同じフォルダーに配置されるようにコンテナーイメージにコピーします
COPY hello_world_http.wasm /hello_world_http.wasm

# エントリーポイントには/spin.tomlを設定します
# 利用するSpinアプリを実行するための低レベルランタイム向けの設定です
ENTRYPOINT [ "/spin.toml" ]