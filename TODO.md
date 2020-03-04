## TODO

- [ ] `P1` Persistence
  - [ ] search/filter history
  - [ ] search/filter config
  - [ ] User configuration
- [ ] `P0`  Base functions
  - [ ] raw-log view window
  - [ ] filter line
    - [ ] search
    - [ ] filter
    - [ ] string
    - [ ] regex
- [ ] `P1` Syntax highlighting
- [ ] `P2` Fancy Requirement
  - [ ] Time range filtering
  - [ ] Jump
  - [ ] Hover tips
- [ ] `P1` Optimization
  - [ ] hold xlog file's hash to avoid repeat decoding
- [ ] `P0` Xlog processing
  - [x] decode xlog file 
    - [ ] Magic_start 0x8(mmap tips)
  - [ ] abs path
    - [x] mac
    - [ ] windows
    - [ ] linux
- [x] `P0` Rust ↔ CPP ↔ Electron
  - [x] mac
  - [x] windows
  - [ ] linux

## 框架
1. xlog文件和decode后的内容都较大，所以log文件[[1]](#1)需要持久化到硬盘上。
2. 在对log文件执行过滤/删除操作的时候会弹出virtual document，重复操作会刷新[[2]](#2)virtual document，另存为[[3]](#3)后可以将过滤结果保存为普通文件。

## 问题
- 将xlog文件解压为xlog.log文件并打开
  - xlog.log文件存在哪里？原目录 or 子目录? `/tmp` or `%TMP%` ?
  - xlog.log文件同名覆盖？或者用后缀区分？
  - xlog.log文件需要清理吗？
    - 如清理，何时清理？激活插件/打开vscode时？
- 过滤操作生成xlog

###### 注

<span id="1">1. 后缀为xlog.log
<span id="2">2. 新建还是刷新？
<span id="3">3. 有无必要增加一个命令/按钮/快捷键来快速保存过滤结果的log？
