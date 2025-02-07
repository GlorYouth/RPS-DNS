# RPS-DNS
A Rust DNS light client/server dedicated to high performance and safe.
Currently, we are developing Resolver. Client and Server just in the future plan.

## 现在阶段
Resolver还处于alpn阶段，勉强可以直接用  
Client/Server还遥遥无期(还没准备dev呢)
目前整体项目架构已经确定下来了，不会再有太大的改变了  
基层的实现效率还是可以的  
欢迎fork and commit  

### Client
#### 正在实施的RFC:
- [ ] RFC1035  

## 待办
- [ ] 解决剩下少数flag，例如TC  
- [ ] 写代码注释，可以交给AI  
- [ ] 将整体的代码实现从alpn阶段逐步转成stable阶段
- [ ] 完善各种DNS类型，目前只实现了A AAAA CNAME，先把RFC1035内有的实现了，类型参考[维基百科](https://en.wikipedia.org/wiki/List_of_DNS_record_types) 
- [ ] 完善Log,例如存储log到本地,定期清空Vec,将log与Error协调起来
- [ ] 实现性能向的Resolver  

## 未来实现
- 实现dns缓存
- 支持EDNS
- 支持DNSSEC
- 支持https,quic,h3,tls等类型请求
- 支持多server下轮询/基于tokio并发
- 支持获取返回最快dns服务器的结构/返回所有结果中最快的ip（详见smart_dns）
- 实现基于tokio的DNSServer
- 实现递归查询
- 支持私人DNS

## 目录树
```
RPS-DNS
├─ Cargo.toml
├─ LICENSE
├─ README.md
├─ src
│  ├─ bench_func.rs
│  ├─ dns.rs
│  ├─ lib.rs
│  ├─ main.rs
│  └─ dns
│     ├─ error.rs
│     ├─ net.rs
│     ├─ resolver.rs
│     ├─ types.rs
│     ├─ utils.rs
│     ├─ utils
│     │  ├─ server_type.rs
│     │  ├─ slice_operator.rs
│     │  └─ slice_reader.rs
│     ├─ types
│     │  ├─ base.rs
│     │  ├─ parts.rs
│     │  ├─ parts
│     │  │  ├─ header.rs
│     │  │  ├─ question.rs
│     │  │  ├─ raw.rs
│     │  │  ├─ record.rs
│     │  │  ├─ request.rs
│     │  │  ├─ response.rs
│     │  │  └─ raw
│     │  │     ├─ header.rs
│     │  │     ├─ question.rs
│     │  │     ├─ record.rs
│     │  │     ├─ request.rs
│     │  │     └─ response.rs
│     │  └─ base
│     │     ├─ dns_type.rs
│     │     └─ domain.rs
│     ├─ net
│     │  └─ query.rs
│     └─ error
│        ├─ error.rs
│        └─ logger.rs
└─ benches
└─ benchmark.rs
```