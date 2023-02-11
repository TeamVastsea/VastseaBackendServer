# 用户模块开发笔记

## code方法登录

### 流程

- 接受登录code
- access_token
- xbl_response
- xsts_response
- xbox_token
- has_game

### 可获取信息

- 绑定的QQ号
- uuid
- mc游戏名
- 用户组

### 适用范围

- 官网仪表盘登录
    1. 用户请求登录
    2. 获取所需要的信息并显示
    3. 保存瀚海token及用户信息（如果用户同意使用cookie，否则不保存）
- 启动器登录
    1. 用户请求登录
    2. 获取所需要的信息
    3. 保存瀚海token，后续下载资源请求均使用瀚海token

### 缺点

需要请求多个微软的api，速度较慢<br>
release请求一次大概需要5s（i9-10850k，32GB内存，北京未挂vpn，成功情况）

## token方法登录