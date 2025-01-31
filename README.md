# RPS-DNS
A Rust DNS light client/server dedicated to high performance and safe.

## 现在阶段
还处于dev阶段，无法直接用  
目前整体项目架构已经确定下来了，不会再有太大的改变了  
基层的实现效率还是可以的  
欢迎fork and commit  

当前主要几个分支不允许直接修改
### Client
#### 正在实施的RFC:
- [ ] RFC1035  

## 待办
- [ ] 实现header中每个flag 对应的检测/行为，需要阅读rfc
- [ ] 实现从最底层开始的#[test]模块，争取全覆盖
- [ ] 写代码注释，可以交给AI
- [ ] 完善各种DNS类型，目前只实现了A AAAA CNAME，而且实现还不完全吧，类型参考[维基百科](https://en.wikipedia.org/wiki/List_of_DNS_record_types) ，一定要实现的是PTR类型，NS可以看情况实现
- [ ] 解决不了解SmallMap和其长期未维护带来的不确定性
- [ ] 完成UDP仅限512字节以内的问题，到底是切片还是换成TCP
- [ ] 完善DNSClient，并配套实现Error

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
