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

- 请求：GET /users?code=[获取的code]&token=[是否需要服务器token]
- 请求说明：
    - code(参考 `https://mccteam.github.io/redirect.html` 中的获取方式)
- 返回：
    - 如果成功登录并获取到用户信息，则返回200和用户信息
    - 如果缺少参数，则返回400
    - 如果用户未拥有mc，则返回401
    - 如遇其他问题，则返回500
- 请求实例：GET /users?code=M.R3_BAY.ae4a08be-40bc-d750-9fb2-15e0c808c543
- 返回实例：{ "_id": "544e8a58c8054879b01ad596d8175dc4", "display_name": "zrll_", "enabled": true, "
  group": ["default"], "bind_qq": null }

### access_token微软登录

- 请求：GET /users?atoken=[access_token]
- 返回：
    - 如果成功登录并获取到用户信息，则返回200和用户信息
    - 如果缺少参数，则返回400
    - 如果用户未拥有mc，则返回401
    - 如遇其他问题，则返回500
- 请求实例：POST /users?atoken=******
- 返回实例：{ "_id": "544e8a58c8054879b01ad596d8175dc4", "display_name": "zrll_", "enabled": true, "
  group": ["default"], "bind_qq": null }

### 瀚海token登录

- 请求：GET /users?htoken=[token]
- 返回：
    - 如果成功登录并获取到用户信息，则返回200和用户信息
    - 如果缺少参数，则返回400
    - 如果用户未拥有mc，则返回401
    - 如遇其他问题，则返回500
- 请求实例：POST /users?htoken=******
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

## 外部API
本模块所有api均需要api key
### API列表
- [x] 封禁用户
- [x] 绑定QQ
- [x] 获取QQ
- [x] 获取幸运值

### 封禁用户

- 请求：PUT /users?uuid=[uuid]&qq=[qq]&reason=[reason]&key=[key]
- reason为选填
- qq与uuid仅需提供一个
- 返回：
  - 如果成功封禁，则返回200
  - 如果缺少参数，则返回400
  - 如果key错误，则返回401
  - 如遇其他问题，则返回500
- 请求实例：PUT /users?uuid=544e8a58c8054879b01ad596d8175dc4&reason=QAQ&key=q2XS6AXzNNMK2ksMDTf7bqxypBEM3q9CQq2WWE4KLOU~

### 绑定QQ

- 请求：PATCH /users?uuid=[uuid]&qq=[qq]&key=[key]
- 返回：
  - 如果成功绑定，则返回200
  - 如果缺少参数，则返回400
  - 如果key错误，则返回401
  - 如遇其他问题，则返回500
- 请求实例：PATCH /users?uuid=544e8a58c8054879b01ad596d8175dc4&qq=2406324685&key=q2XS6AXzNNMK2ksMDTf7bqxypBEM3q9CQq2WWE4KLOU~

### 获取QQ
- 请求：GET /user/qq?uuid=[uuid]&key=[key]
- 返回：
  - 如果成功获取，返回200
  - 如果缺少参数，返回400
  - 如果key错误，返回500
  - 如果用户不存在，返回500
  - 如果发生其他错误，返回500
- 请求实例：GET /user/qq?uuid=544e8a58c8054879b01ad596d8175dc4&key=q2XS6AXzNNMK2ksMDTf7bqxypBEM3q9CQq2WWE4KLOU~

### 获取幸运值
- 请求：GET /user/luck?uuid=[uuid]&key=[key]
- 返回：
  - 如果成功获取，返回200
  - 如果缺少参数，返回400
  - 如果key错误，返回500
  - 如果用户未绑定，返回500
  - 如果发生其他错误，返回500
- 请求实例：GET /user/qq?uuid=544e8a58c8054879b01ad596d8175dc4&key=q2XS6AXzNNMK2ksMDTf7bqxypBEM3q9CQq2WWE4KLOU~