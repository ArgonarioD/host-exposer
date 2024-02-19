<h1 align="center">Host Exposer</h1>

<p align="center">一个能够简单并且高效地将多个可能隐藏在 NAT 或防火墙后的客户端机器的所有主机地址（包括 IPv4 和 IPv6 地址）通过一个服务端展示的工具。</p>

<div align="center">
<a href="LICENSE"> 
    <img src="https://img.shields.io/github/license/ArgonarioD/host-exposer" alt="License">
</a>
</div>

<p align="center">
<a href="README.md">English</a> | 简体中文
</p>

## 使用

1. 在服务端运行 `host-exposer-server`（并通过命令行参数指定认证密码 或 使用每次启动自动生成的随机密码）
2. 在客户端运行 `host-exposer-client`，并通过命令行参数指定服务端的 Websocket URI（默认情况下，监听在 3030 端口，Websocket URI 位于 `/expose`），如：
    
   ```sh
   host-exposer-client ws://your-server-ip:3030/expose -p 'your-password'
   ```
3. 通过浏览器访问服务端监听的端口，输入密码，即可查看所有已连接到服务端的客户端的主机地址。
