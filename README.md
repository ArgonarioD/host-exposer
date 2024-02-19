<h1 align="center">Host Exposer</h1>

<p align="center">A tool that allows you to display multiple host addresses of client machines (including both IPv4 and IPv6) that may be behind a firewall or a NAT to a server simply and resource-efficiently.</p>

<div align="center">
<a href="LICENSE"> 
    <img src="https://img.shields.io/github/license/ArgonarioD/host-exposer" alt="License">
</a>
</div>

<p align="center">
English | <a href="README_zh_CN.md">简体中文</a>
</p>

## Usage

1. Run `host-exposer-server` on the server machine (specify the authentication password via command line arguments or use a randomly generated password each time you start the server).
2. Run `host-exposer-client` on the client machine, and specify the server's Websocket URI via command line arguments (by default, it listens on port 3030, and the Websocket URI is at `/expose`), for example:

   ```sh
   host-exposer-client ws://your-server-ip:3030/expose -p 'your-password'
   ```

3. Access the port that the server is listening on via a web browser, enter the password, and you can view all the host addresses of the clients that have connected to the server.