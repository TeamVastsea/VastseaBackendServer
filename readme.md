# 瀚海工艺-Vastsea | 后端API使用指南

- API遵守RESTful
- 返回的内容在body里面

## 用户模块

### API列表

- [x] code微软登录
- [x] access_token微软登录
- [x] 瀚海token登录
- [ ] 刷新Minecraft信息
- [ ] 删除账户

### code微软登录

- 请求：`GET /users?code=[获取的code]&token=[是否需要服务器token]`
- 请求说明：
    - code(参考 `https://mccteam.github.io/redirect.html` 中的获取方式)
- 返回：
    - 如果成功登录并获取到用户信息，则返回`200`和用户信息
    - 如果缺少参数，则返回`400`
    - 如果用户未拥有Minecraft正版账号，则返回`401`
    - 如遇其他问题，则返回`500`
- 请求实例：`GET /users?code=M.R3_BAY.ae4a08be-40bc-d750-9fb2-15e0c808c543`
- 返回实例：
  ```json
  { "_id": "544e8a58c8054879b01ad596d8175dc4", "display_name": "zrll_", "enabled": true, "group": ["default"], "bind_qq": null }
  ```

### access_token微软登录

- 请求：`GET /users?atoken=[access_token]`
- 返回：
    - 如果成功登录并获取到用户信息，则返回`200`和用户信息
    - 如果缺少参数，则返回`400`
    - 如果用户未拥有Minecraft正版账号，则返回`401`
    - 如遇其他问题，则返回`500`
- 请求实例：`POST /users?atoken=******`
- 返回实例：
  ```json
  { "_id": "544e8a58c8054879b01ad596d8175dc4", "display_name": "zrll_", "enabled": true, "group": ["default"], "bind_qq": null }
  ```

### 瀚海token登录

- 请求：`GET /users?htoken=[token]`
- 返回：
    - 如果成功登录并获取到用户信息，则返回`200`和用户信息
    - 如果缺少参数，则返回`400`
    - 如果用户未拥有mc，则返回`401`
    - 如遇其他问题，则返回`500`
- 请求实例：`POST /users?htoken=******`
- 返回实例：
  ```json
  { "_id": "544e8a58c8054879b01ad596d8175dc4", "display_name": "zrll_", "enabled": true, "group": ["default"], "bind_qq": null }
  ```

## 新闻模块

新闻id为正整数（可能不连续）
带`*`的需要管理员权限

### API列表

- [x] 获取新闻列表
- [x] 获取新闻详细信息
- [x] *上传新闻

### 获取新闻列表

- 返回开始id及其之后n-1个新闻的信息
- 请求：`GET /news?start=[开始nid]&n=[获取个数]`

### 获取新闻详细信息

- 请求：`GET /news/[id]`
- 返回md文件内容

### 上传新闻

- 请求：`POST /news`
- 请求body格式：`{"token": "瀚海token", "body": "新闻具体内容", "info": {"title": "新闻标题", "description": "新闻简介"}}`
- 返回：创建成功200，鉴权失败401，创建失败500
- 请求body示例`{"token": "token", "body": "this is the first", "info": {"title": "111", "description": "first"}}`

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

- 请求：`PUT /users?uuid=[uuid]&qq=[qq]&reason=[reason]&key=[key]`
- reason为选填
- qq与uuid仅需提供一个
- 返回：
  - 如果成功封禁，则返回`200`
  - 如果缺少参数，则返回`400`
  - 如果key错误，则返回`401`
  - 如遇其他问题，则返回`500`
- 请求实例：`PUT /users?uuid=544e8a58c8054879b01ad596d8175dc4&reason=QAQ&key=q2XS6AXzNNMK2ksMDTf7bqxypBEM3q9CQq2WWE4KLOU~`

### 绑定QQ

- 请求：`PATCH /users?uuid=[uuid]&qq=[qq]&key=[key]`
- 返回：
  - 如果成功绑定，则返回`200`
  - 如果缺少参数，则返回`400`
  - 如果key错误，则返回`401`
  - 如遇其他问题，则返回`500`
- 请求实例：`PATCH /users?uuid=544e8a58c8054879b01ad596d8175dc4&qq=2406324685&key=q2XS6AXzNNMK2ksMDTf7bqxypBEM3q9CQq2WWE4KLOU~`

### 获取QQ
- 请求：`GET /user/qq?uuid=[uuid]&key=[key]`
- 返回：
  - 如果成功获取，返回`200`
  - 如果缺少参数，返回`400`
  - 如果key错误，返回`500`
  - 如果用户不存在，返回`500`
  - 如果发生其他错误，返回`500`
- 请求实例：`GET /user/qq?uuid=544e8a58c8054879b01ad596d8175dc4&key=q2XS6AXzNNMK2ksMDTf7bqxypBEM3q9CQq2WWE4KLOU~`

### 获取幸运值
- 请求：`GET /user/luck?uuid=[uuid]&key=[key]`
- 返回：
  - 如果成功获取，返回`200`
  - 如果缺少参数，返回`400`
  - 如果key错误，返回`500`
  - 如果用户未绑定，返回`500`
  - 如果发生其他错误，返回`500`
- 请求实例：`GET /user/qq?uuid=544e8a58c8054879b01ad596d8175dc4&key=q2XS6AXzNNMK2ksMDTf7bqxypBEM3q9CQq2WWE4KLOU~`