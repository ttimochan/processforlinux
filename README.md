# Process Report For Linux

## 1. 介绍

Mix Space 适用于 Linux 桌面系统的进程上报工具，使用 Rust 实现，内存占用小，实现粗糙，暂时没有观察到内存泄漏，是 [ProcessReporterMac](https://github.com/mx-space/ProcessReporterMac) 的 Linux 版本，目前仅有 CLI。

在 ArchLinux KDE 上开发。

## 2. 安装

### 2.1 前置

环境要求：

- Xorg(X11)，暂时不支持 Wayland
- 系统进程间通信使用 D-bus

你的 Linux 操作系统需要有 `xprop`, `xwininfo` 命令。

如果你是 ArchLinux 用户，可以使用以下命令安装：

```bash
yay -S xorg-xprop xorg-xwininfo
```

其他发行版本，具体安装方法请参考你的 Linux 发行版的文档。

### 2.2 安装

### 2.2.1 从预编译包安装

从 [Release](https://github.com/ttimochan/processforlinux/releases) 页面下载对应的版本，Arch Linux 可以使用 `yay -U xxx.pkg.tar.zst` 安装，其他发行版本将 `processforlinux-amd64.tar.gz` 压缩包下载，解压后将可执行文件放到你喜欢的位置，给予可执行权限。

如果使用 `ldd` 命令查看链接库，如果发现链接库缺失，请从源码开始安装。

### 2.2.2 从源码安装

如果你是 Ubuntu / Debian ，请先安装以下依赖：

```bash
sudo apt libdbus-1-dev pkg-config libssl-dev gcc -y
```

安装 Rust 环境

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

编译

```bash

git clone https://github.com/ttimochan/processforlinux.git

cd processforlinux

cargo build --release
```

在 /target/release 目录下生成可执行文件 processforlinux，放到你喜欢的位置，给予可执行权限。

## 3. 使用

### 3.1 配置文件

创建 .env.process 文件，内容可以参照 .env.process.example ：

```sh
# 你的 key
API_KEY=your_key
# 你的云函数地址
API_URL=https://api.example.cn/api/v2/fn/ps/update
# 上报时间间隔，单位为秒
REPORT_TIME=30
# 是否开启媒体状态上报
MEDIA_ENABLE=true 
# 是否打印日志
LOG_ENABLE=true 
```

### 3.2 运行

二进制文件只需要放在执行目录即可，例如：

```bash
sudo cp processforlinux /usr/local/bin
```

例如你在 /home/timochan/ 下执行，那么配置文件就放在 `/home/timochan/.env.process`，会自动读取当前执行目录的配置文件，如：

```bash
processforlinux
```

使用短参数来指定配置文件的位置，如：

```bash
processforlinux -c /path/to/.env.process
```

当然长参数也可以，如：

```bash
processforlinux --config /path/to/.env.process
```

后台运行：

```bash
nohup processforlinux -c /path/to/.env.process &
```

### 3.3 关于日志

你可以使用重定向符号来将日志输出到文件，如：

```bash
processforlinux -c /path/to/.env.process > /path/to/processforlinux.log
```

## 4. 问题

- 媒体上报功能在 KDE 上测试通过，其他桌面环境未测试，目前仅兼容了网易云音乐和 yesplaymusic，QQ 音乐没有固定的 D-bus 路径，没办法获取信息。

## 5. 其他平台

- [Windows](https://github.com/TNXG/ProcessReporterWinpy)
- [macOS](https://github.com/mx-space/ProcessReporterMac)
