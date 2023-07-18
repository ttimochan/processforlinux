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

### 3.1 配置文件

创建 .env.process 文件，内容可以参照 .env.process.example ：

```sh
API_KEY=your_key  # 你的 key
API_URL=https://api.example.cn/api/v2/fn/ps/update # 你的 API 地址
REPORT_TIME=30 # 上报时间间隔，单位为秒
MEDIA_ENABLE=true # 是否开启媒体状态上报
LOG_ENABLE=true # 是否打印日志
```

当你填写完毕，请把注释删除，并剔除多余的空格，否则会报错（读入配置文件的逻辑很简单，没做意料之外的错误处理）。

### 3.2 运行

你可以把配置文件和知悉文件放在同一目录下，然后运行：

```bash
./processforlinux
```

当然使用短参数来指定配置文件的位置，如：

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

## 4. 其他平台

- [Windows](https://github.com/TNXG/ProcessReporterWinpy)
- [macOS](https://github.com/mx-space/ProcessReporterMac)
