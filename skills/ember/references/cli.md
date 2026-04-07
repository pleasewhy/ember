# CLI 文档

本文说明如何使用 `ember-cli` 创建、构建、调试、发布和维护 Wasm HTTP worker。

## 1. CLI 能做什么

当前主要覆盖：

- 初始化一个新的 worker 项目
- 本地构建 Wasm 产物
- 本地启动 HTTP 调试服务
- 登录兼容控制面
- 发布、部署、回滚、查询状态
- 管理环境变量、secret 和 SQLite 备份

## 2. 安装

```bash
cargo install --git https://github.com/pleasewhy/ember ember-cli
```

如果你还需要查看源码或 examples：

```bash
git clone https://github.com/pleasewhy/ember.git
cd ember
```

## 3. 最小工作流

```bash
ember init hello-worker
cd hello-worker
rustup target add wasm32-wasip2
ember build
ember dev --addr 127.0.0.1:3000
```

访问：

```bash
curl http://127.0.0.1:3000/
```

## 4. 配置文件

CLI 登录后会把凭据写到本地配置目录：

```text
$XDG_CONFIG_HOME/ember/config.toml
```

兼容旧链路时，还会回退尝试：

- `$XDG_CONFIG_HOME/ember/config.toml`
- `$XDG_CONFIG_HOME/wkr/config.toml`

临时调试建议使用独立配置目录：

```bash
XDG_CONFIG_HOME=/tmp/ember-cli-config ember whoami
```

## 5. 命令说明

### `ember init <path>`

初始化一个新的 worker 项目。

```bash
ember init hello-worker
```

### `ember build`

构建当前 worker 的 Wasm 产物。

- 优先尝试 `cargo component build`
- 否则回退到 `cargo build --target wasm32-wasip2`

```bash
ember build
ember build --manifest ./worker.toml
```

### `ember dev`

本地启动一个 HTTP 调试服务：

```bash
ember dev --addr 127.0.0.1:3000
ember dev --skip-build --addr 127.0.0.1:3000
```

### `ember login`

```bash
ember login
```

CLI 会启动一个临时 localhost 回调地址，打开 embercloud 浏览器登录页，并在授权完成后把 CLI token 写入本地 config。

### `ember whoami`

```bash
ember whoami
```

### `ember logout`

```bash
ember logout
```

### `ember app`

所有云端 app 维度操作现在统一收敛到 `ember app ...`：

- `ember app list`
- `ember app create`
- `ember app status`
- `ember app publish`
- `ember app deploy`
- `ember app deployments`
- `ember app events`
- `ember app logs`
- `ember app rollback`
- `ember app delete-version`
- `ember app delete`
- `ember app env ...`
- `ember app secrets ...`
- `ember app sqlite ...`

### `ember app publish`

```bash
ember app publish
ember app publish --manifest ./worker.toml
```

### `ember app deploy <app> <version>`

```bash
ember app deploy hello-worker <version>
```

### 查询命令

- `ember app list`
- `ember app status <app>`
- `ember app deployments <app> --limit <n>`
- `ember app events <app> --limit <n>`
- `ember app logs <app> --limit <n>`

### 回滚和删除

```bash
ember app rollback hello-worker <old-version>
ember app delete-version hello-worker <version>
ember app delete hello-worker
```

## 6. 环境变量和 Secret

环境变量：

```bash
ember app env list hello-worker
ember app env set hello-worker APP_ENV production
ember app env delete hello-worker APP_ENV
```

Secret：

```bash
ember app secrets list hello-worker
ember app secrets set hello-worker openai-api-key <secret-value>
ember app secrets delete hello-worker openai-api-key
```

## 7. SQLite 备份与恢复

```bash
ember app sqlite backup hello-worker ./backup.sqlite3
ember app sqlite restore hello-worker ./backup.sqlite3
```

## 8. 组件签名发布

发布前可设置：

- `EMBER_SIGNING_KEY_ID`
- `EMBER_SIGNING_KEY_BASE64`

兼容旧链路时也接受：

- `WKR_SIGNING_KEY_ID`
- `WKR_SIGNING_KEY_BASE64`

```bash
ember app publish
```

## 9. 常见问题

### `ember build` 提示缺少 `wasm32-wasip2`

```bash
rustup target add wasm32-wasip2
```

### `ember app publish` 提示找不到 artifact

```bash
ember build
```

并检查 `worker.toml` 里的 `component` 路径是否正确。

### `ember dev` 启动了但访问不到接口

先确认：

- 监听地址是否正确
- 产物是否已经构建成功
- 路由是否正确匹配
- 如果设置了 `base_path`，请求路径是否带上前缀
