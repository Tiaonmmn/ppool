# ppool (WIP)

受 [proxy_pool](https://github.com/jhao104/proxy_pool) 启发完成的 IP 代理池

## 安装

需要 nightly 版本 (谁让 rocket 只能在 nightly 下工作呢...

```bash
cargo install --git https://github.com/Aloxaf/ppool
```

## 运行

```bash
RUST_LOG=INFO ppool
```

注: 可通过环境变量 `ROCKET_PORT` 来控制端口, 默认 8000

## 配置

### config.toml

主要配置, 可通过 `ppool -C > config.toml` 生成默认配置, 

`ppool -c config.toml` 使用指定配置

### Rocket.toml

服务器配置, 完整示例如下

实际使用中只需要列出要修改的值即可

```toml
[production]
address = "0.0.0.0"
port = 80
workers = 10
log = "critical"
secret_key = "8Xui8SN4mI+7egV/9dlfYYLGQJeEx4+DwmSQLwDVXJg="
limits = { forms = 32768 }
```

## 优点

- 功能更全面: 记录代理的质量, 类型等数据. 接口更完善
- 依赖少, 资源消耗更少 (大概
- 更高性能 (大概
- 只有一个 elf/exe, 不觉得很 coooool 吗! (这就是我最开始的想法

## 缺点

- 扩展麻烦了点...复杂的规则目前只能硬编码进代码
- 代理太多目测遭不住

## TODO

- [x] 基本框架
- [x] 记录进度
- [x] 更多代理
- [x] 更多注释
- [x] 更多参数 / 配置文件
- [x] 更多代理的信息 <s>(响应速度?)</s>
- [x] 更多线程
- [x] 更完善的错误处理(现在到处都是unwrap)
- [x] 更完善的接口(能够根据需求预筛选)
- [ ] 更少 clone (更高性能)
- [ ] 更高并发性能
- [x] 更好看的变量名 (
- [x] 更方便地修改配置 (reload api)
- [ ] 异步 (其实没用过, 只是先放在这里
- [x] 通过代理爬取代理
- [x] 通过配置文件定义一些简单的爬虫
- [ ] 使用嵌入式数据库 (不可能的, 顶天几千个代理, 要啥数据库
- [ ] 代码写得太丑了啥时候重构下 (不可能的, 能跑就行