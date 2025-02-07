# RPS-DNS
A Rust DNS light client/server dedicated to high performance and safe.

## 现在阶段
还处于alpn阶段，勉强可以直接用  
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
