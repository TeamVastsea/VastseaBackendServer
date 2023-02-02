# API使用指南
- API遵守RESTful
- 返回的内容在body里面
## 用户模块
### API列表
- [ ] code微软登录
- [ ] 账号密码微软登录
- [ ] token登录
- [ ] 刷新mc信息
- [ ] 绑定QQ
- [ ] 删除账户信息


### code微软登录
- 请求：GET /login_code?code=获取的code&token=是否需要服务器token
- 请求说明：
  - code(参考 `https://mccteam.github.io/redirect.html` 中的获取方式)
- 返回：
  - 如果成功登录并获取到用户信息，则返回200和用户信息
  - 如果缺少参数，则返回400
  - 如果用户未拥有mc，则返回401
  - 如遇其他问题，则返回500
- 请求实例：GET /login_code?code=M.R3_BAY.ae4a08be-40bc-d750-9fb2-15e0c808c543
- 返回实例：{"uuid": "544e8a58c8054879b01ad596d8175dc4", "username": "zrll_", "qq": null, "token": null}
- 进度：80%

### 账号密码微软登录
- 请求：POST /password
### body:
{"username":"微软用户名","password":"微软密码"[,"token":bool]}
- 返回：
  - 如果成功登录并获取到用户信息，则返回200和用户信息
  - 如果缺少参数，则返回400
  - 如果用户未拥有mc，则返回401
  - 如遇其他问题，则返回500
- 请求实例：POST /password
### body:
{"username":"test@outlook.com","password":"test"}
- 返回实例：{"uuid": "544e8a58c8054879b01ad596d8175dc4", "username": "zrll_", "qq": null, "token": null}
- 进度：70%

