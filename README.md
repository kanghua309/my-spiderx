# `Scaffold Structure 说明`

脚手架是为嵌入程序所设计框架！！！
满足功能：
- 支持logging（借助defmt） #见 https://defmt.ferrous-systems.com/istr.html
- 支持no_std下的单元测试（借助 defmt-test 框架）
- 支持cargo embed （见enbed.toml），会自动建立 GDB stub）
- 支持splitable 方式的管理多包架构（见https://ferrous-systems.com/blog/test-embedded-app/）

## 依赖
- cargo install flip-link
- cargo install probe-run
- cargo install embed
- rustup target add thumbv7em-none-eabihf #for macbit2

## 用法

### Flash && Run
#### usage - 1
- cargo build
- cargo embed --target thumbv7em-none-eabihf # cargo embed 用法 https://github.com/probe-rs/cargo-embed
#### usage - 2
- probe-run # probe-run 用法https://github.com/knurling-rs/probe-run
#### usage - 3
- cargo run #间接调用 probe run

### Test 
cd cross/self-tests目录执行
-  cargo test --test s90 #执行tests/s90.rs
-  cargo test --test s90_driver #执行tests/s90_driver.rs



## 目录结构
我们采取内外层crate模式，cross 内层！
- cross 目录下是跨平台的内容 ，目前分3+1个，💰 依次向下依赖
- - app 是main程序
- - board、driver 都是lib库
- - self-test 是单独成包的测试程序所在（用于cargo test 在no std下测试）
- cross 外目录见 https://ferrous-systems.com/blog/test-embedded-app/




## issue?
get_duty 返回值和设置的不一样？
period 啥意思？ 50hz 为何不动？ 
set_duty 啥意思？
