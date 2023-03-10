# API使用指南

- API遵守RESTful
- 返回的内容在body里面

## 用户模块

### API列表

- [x] code微软登录
- [x] access_token微软登录
- [x] 瀚海token登录
- [ ] 刷新mc信息
- [ ] 删除账户

### code微软登录

- 请求：GET /user?code=[获取的code]&token=[是否需要服务器token]
- 请求说明：
    - code(参考 `https://mccteam.github.io/redirect.html` 中的获取方式)
- 返回：
    - 如果成功登录并获取到用户信息，则返回200和用户信息
    - 如果缺少参数，则返回400
    - 如果用户未拥有mc，则返回401
    - 如遇其他问题，则返回500
- 请求实例：GET /user?code=M.R3_BAY.ae4a08be-40bc-d750-9fb2-15e0c808c543
- 返回实例：{ "_id": "544e8a58c8054879b01ad596d8175dc4", "display_name": "zrll_", "enabled": true, "
  group": ["default"], "bind_qq": null }

### 账号密码微软登录

- 请求：GET /user?atoken=[access_token]
- 返回：
    - 如果成功登录并获取到用户信息，则返回200和用户信息
    - 如果缺少参数，则返回400
    - 如果用户未拥有mc，则返回401
    - 如遇其他问题，则返回500
- 请求实例：POST /user?atoken=******
- 返回实例：{ "_id": "544e8a58c8054879b01ad596d8175dc4", "display_name": "zrll_", "enabled": true, "
  group": ["default"], "bind_qq": null }

### 账号密码微软登录

- 请求：GET /user?htoken=[token]
- 返回：
    - 如果成功登录并获取到用户信息，则返回200和用户信息
    - 如果缺少参数，则返回400
    - 如果用户未拥有mc，则返回401
    - 如遇其他问题，则返回500
- 请求实例：POST /userd?htoken=******
- 返回实例：{ "_id": "544e8a58c8054879b01ad596d8175dc4", "display_name": "zrll_", "enabled": true, "
  group": ["default"], "bind_qq": null }

## 文件模块

### API列表

- [ ] 检查最新版本
- [ ] 请求下载文件

## 问卷模块

本模块中所有标记`-`的需要管理员权限<br>
所有标记`*`的需要本人或管理权限

### API列表

- [ ] -判卷
- [ ] -维护某一问卷题目与答案
- [ ] *检查某一用户答卷进度
- [ ] *访问某一特定答卷
- [ ] *请求开始问卷或重新答题
- [ ] *提交答案

## 机器人API
本模块所有api均需要颁发给机器人的api key
### API列表
- [ ] 封禁用户
- [x] 绑定QQ

### 绑定QQ

- 请求：PATCH /user?name=[name]&qq=[qq]&key=[key]
- 返回：
  - 如果成功绑定，则返回200
  - 如果缺少参数，则返回400
  - 如果key错误，则返回401
  - 如遇其他问题，则返回500
- 请求实例：POST /user?name=zrll_&qq=2406324685&key=q2XS6AXzNNMK2ksMDTf7bqxypBEM3q9CQq2WWE4KLOU=