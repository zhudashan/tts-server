# tts-server

- 本项目使用的是Edge浏览器“大声朗读”功能的接口，不保证后续可用性和稳定性。


- 项目使用持续 websocket 极大乱提升了请求的并发行以及请求速度，减少了频繁使用http到 websocket 升级协议握手的时间


- 代码稍微有点乱，初学 rust 的玩具，介意的话请移步

使用介绍的话没工夫写，可以使用程序里面的 --help ，或者也可以看看 help.md 文件


如果有人愿意来写 README.md 的话直接提 PR


且行且珍惜

## 项目动态

2022-06-16：Edge 浏览器提供的接口现在已经不能设置讲话风格了，简单点说就是 style 参数废了，传什么都默认了

