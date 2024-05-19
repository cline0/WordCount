# Word Count
## 介绍
- 一个简单的 word count 工具
- 只能进行统计以**空格**、**制表符（`\t`）** 和 **逗号（`,`）** 分隔的文档
## 使用
### 环境要求：
- rust 环境（可以使用 https://rsproxy.cn/ 来进行环境配置，相比于官网，速度较快）
### 构建步骤
1. clone 本项目
```bash
    git clone https://github.com/cline0/WordCount.git
```
2. 构建并使用
```bash
    cargo build -- --help
    cargo run  -- -w -c test.txt
```
> cargo 会将 `--` 之后的参数传给编译出来的产物（word-count.ext）
## 测试
对于 test.txt 我的程序的结果：

![ ](./images/word-count-test.png)

wc 给出的结果：

![ ](./images/wc-test.png)