# Skyward / songyuan（REDMI K100 Pro Max）更新日志

> 本 fork 基于 kirklin/skyward，针对 songyuan 设备做了大量功能、修复与界面改动。
> 时间均为 Asia/Shanghai。按阶段倒序排列。

---

## 2026-11-15（会话三）— 13 个提交

### 新增

- **WLAN「开机自动打开」开关**（WLAN 页第三行，默认关闭）。打开后开机自动加载 WiFi 驱动并启动 wpa_supplicant；若「密码存在 data 分区」关着（密码在 persist），解密前即可联网。
- **重启页滑动条跟随目标**：显示「滑动以重启到系统 / 恢复模式 / Bootloader / EDL / 关机」，EDL 标红，Bootloader 使用小米橙 `#FF6900`；切换目标时滑块自动回位。
- **错误提示支持换行**：底部提示胶囊限制在屏幕宽度内自动换行、高度自适应，不再横向超出屏幕。
- **解密页错误原因可滚动**：`未能打开 data 分区` 下面的长原因改为可滚动长页，不再被屏幕底边截断。

### 变更

- **Fastboot 弃用 fastbootd，改走原厂 bootloader**：`adb reboot fastboot` 和重启页「Bootloader」都直接进入原厂 fastboot（能刷物理分区；不再支持刷 super 内逻辑分区）。重启页删掉「Fastboot（用户空间 fastbootd）」单独一项。
- **错误提示显示时间**从 2 秒加到 4 秒。
- **解密相关错误中文化**：锁屏密码错误、密码错误次数过多（含剩余秒数）、密钥/凭据/weaver/启动控制服务未启动，均显示中文。
- **关于 Skyward**：设置页「关于」分组只保留「关于 Skyward」一行（去掉「关于本机」和版本提示）；页面「编码 agent」「底层模型」两卡去掉小灰字；项目优化者版本号改为 `0.0.2`。
- **WLAN 说明文字**简化：「开启后，可连接 WLAN 下载系统更新。」（去掉 Skyward 新版本）；密码存放说明加大与开关列表的间距并缩短文字。

### 修复

- **WLAN 开机自动打开时 `wlan0` 上电 `EBUSY`**：驱动创建接口后自身初始化未完成，`bring_up` 增加重试（最多 20 秒），已实测 `wlan0 up` 且 supplicant 正常运行。
- **恢复 11月9日被 `e4edf4c` 覆盖丢失的解密页优化**：解密页去掉左上角返回箭头、作为启动第一页、主页解密行「未解密显示/已解密隐藏」、加载中淡色键盘、解密成功自动跳主页。
- **mtp,adb 配置下 adbd 未启动**：`usb::set` 在配置含 adb 且 adbd 未运行时手动拉起 adbd。
- 顺带修复 `skyward-preview` 中残留的旧 `data_waiting` 双参数调用。

---

## 2026-11-15（会话一、二）— v0.3.1 合并后的修复与验证

- **触摸振动修复**（`19ce346`）：点击振动恢复，38 处测试断言同步，348 passed。
- **fastboot 调查与初步修复**（`2e423df`）：等 boot control HAL 注册后再要端口；后续查明 Fastboot 路径缺 installed system 库导致 HAL 崩溃（该方案最终被「改走原厂 bootloader」取代）。
- **解密链修复**：weaver / secure_element / OMAPI bridge 服务启动顺序与参数修复（`devices/songyuan.toml`），解密恢复可用。
- **v0.3.1 升级**：`git merge up-v0.3.1`（`388c686`），36 个冲突全部解决，重放 fork 全部改动；之后修 v0.3.1 首刷崩溃（异步 ready 回归）与解密失败。

---

## 2026-11-14 — fork 迁移到 kirklin/skyward v0.3.1

- `e4edf4c`：把 fork 移到 v0.3.1 结构（`app/` 模块化重构等）。
- `d85acab`：新页面上保留「Wi-Fi 密码存哪」开关。
- `96b8101`：补回被移除的 bridge 所需的配置段。

---

## 2026-11-09 — fork 大版本改动（v0.2.x）

### 解密 / data 分区

- 修复 /data 解密链路：qseecomd、gatekeeper、keymint、secure_element、weaver、OMAPI bridge 服务按顺序配置（`devices/songyuan.toml`）。
- 解密页：去掉左上角返回箭头、设为启动第一页、加载中键盘淡色不可按、解密成功自动跳主页、删除「data 分区已解密」中间页。
- 主页「解密」行：未解密时显示可点击，解密完成后整行去掉（不是只藏箭头）。

### WLAN

- **记忆 WiFi 密码**：连接过的网络把密码写盘（persist 或 data，可选），下次开机自动连接。
- WiFi 连接提速：状态轮询 5s→1s、记住密码后不等全量扫描直接连、密码文件三列格式（SSID/密码/加密方式）。
- 网络时间同步：连上 WiFi 后可校时，状态栏时间跳变时整页刷新。

### 界面

- 主界面设备卡去掉「当前槽位 / 数据分区 / 可用空间 / 电量」读数条。
- 设置页底部「Skyward 更新」改成「关于 Skyward」，页面改为四张纯展示卡（项目原作者、项目优化者、编码 agent、底层模型），无任何按钮。
- 振动：页面打开/关闭的轻振动（`19ce346`）。

### 启动与性能

- ADSP 等待白等 10 秒修复（ADSP 不在时不再空等）。
- 服务就绪等待改为异步线程（不阻塞 UI 启动）。
- 触摸屏唤醒后重新下发位置上报（retell，150ms 延时）。
- 字体子集化：MiSansVF 20.0 MB → 10.7 MB（保留 ASCII/GB2312/Big5/仓库字符）。
- `omapi-bridge` strip：5.95 MB → 786 KB。

### 移除

- **Skyward 自更新**：检查 GitHub 新版本、下载、校验、写入 recovery 分区整套代码移除（关于页不再联网）。
- **双系统 UI 与功能**：入口全部去掉（`dual_boot = false`），底层 crate 保留。

### 工程整理

- `cargo clippy` 零警告；`cargo fmt` 不跑（原作者手写风格）。
- `devices/songyuan.toml` 注释英文化（与上游风格一致）。
- 清理仓库垃圾文件（0 字节文件、`.bak` 备份等）。

---

## 2026-11-08 及更早 — v0.2.x 基座

- 基于 TUSB-5834 的 skyward fork（v0.2.2 结构）适配 songyuan。
- 救砖：`fastboot flash misc zero4m.img` + `fastboot --set-active=b`，用户数据保住。
- 基础适配：触摸（FocalTech）、振动（qcom-hv-haptics）、显示、adbd、MTP。

---

## 当前状态

- 镜像：`skyward-songyuan-fix17.img`（MD5 `fcdcb05dba5d52411ae6b7a4e13671c5`）
- 设备：recovery_b 槽，Skyward v0.3.1（fork）
- 测试：`cargo test --workspace` 893 passed（2026-11-20 复核）
- 已知限制：原厂 bootloader 无法刷 super 内逻辑分区；如需刷系统分区请用完整 OTA 包走「安装」页。
