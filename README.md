### Code-Search

> 一个可以在命令行中使用的代码搜索工具
```
Usage: code-search [OPTIONS] --key <Key>

Options:
  -p, --path <Path>          搜索路径，文件或目录 [default: .]
  -l, --language <Language>  使用语言文件扩展名，如 rs、md等
  -s, --symbol               只搜索符号，如类名、函数名称等
  -k, --key <Key>            关键字
  -h, --help                 Print help
```
symbol 搜索支持以下语言：
1. rust
2. java
3. python
4. c
5. c++
6. c#
7. javascript
8. go
