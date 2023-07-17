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

### 2.2 编译

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

## 3. 使用

在 /target/release 目录下生成可执行文件 processforlinux，放到你喜欢的位置，给予可执行权限。

然后将 .env.process.example 文件重命名为 .env.process，填写你的 Mix Space 项目的 API_URL 和你设置的 API_KEY。
该文件是配置文件可以与二进制文件放在同一目录下。当然也可以放在其他目录，但是需要在运行时指定配置文件的路径。

```bash
./processforlinux -c /path/to/.env.process
```

当然长参数也可以，如：

```bash
./processforlinux --config /path/to/.env.process
```

后台运行

```bash
nohup ./processforlinux -c /path/to/.env.process &
```

等该项目稳定后将会提供二进制文件。
