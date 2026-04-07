# `worker.toml`

`worker.toml` 是 Ember worker 的配置文件。它描述：

- worker 名称
- Wasm 构件路径
- 基础路径
- 环境变量和 secret 绑定
- SQLite、资源限制、出站网络策略
- embercloud 发布时对应的 app

当前支持的字段以 `ember-manifest` 为准，完整文档见：

- `docs/worker-toml.md`

核心字段：

- `name`
- `component`
- `base_path`
- `[env]`
- `[secrets]`
- `[sqlite].enabled`
- `[resources].cpu_time_limit_ms`
- `[resources].memory_limit_bytes`
- `[network].mode`
- `[network].allow`
- `[embercloud].app`

最常见示例：

```toml
name = "hello-worker"
component = "target/wasm32-wasip2/release/hello_worker.wasm"
base_path = "/"

[env]
APP_NAME = "hello-worker"

[secrets]
OPENAI_API_KEY = "secret://openai-api-key"

[sqlite]
enabled = true

[resources]
cpu_time_limit_ms = 5000
memory_limit_bytes = 134217728

[network]
mode = "allow_list"
allow = ["api.openai.com:443"]

[embercloud]
app = "hello-worker"
```

关键规则：

- `name` 和 `embercloud.app` 最长 48 个字符，只允许字母、数字、`-`、`_`
- `base_path` 必须以 `/` 开头
- `[env]` 和 `[secrets]` 的 key 只能使用 `A-Z`、`0-9`、`_`
- `network.allow` 只在 `mode = "allow_list"` 时生效
- `resources` 里的数值如果设置，必须大于 0
