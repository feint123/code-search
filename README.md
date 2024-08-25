### Code-Search

> 一个可以在命令行中使用的代码搜索工具
```
Usage: code-search [OPTIONS] --key <Key>

Options:
  -p, --path <Path>          搜索路径，文件或目录 [default: .]
  -l, --language <Language>  使用语言文件扩展名，如 rs、md等
  -s, --symbol               只搜索符号，如类名、函数名称等
  -k, --key <Key>            关键字
  -r, --reg                  启用正则表达式（会减缓搜索速度）
  -i, --interactive          启用交互模式（该模式会构建索引，请指定具体的项目目录）
  -h, --help                 Print help
  -V, --version              Print version
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

#### 交互模式

> 支持符号搜索，查看文件大纲

```
❯ code-search -i
当前根路径为 : /Volumes/T7/Github/code-search
>> main
main(/Volumes/T7/Github/code-search/examples/main.c:23)
main(/Volumes/T7/Github/code-search/examples/main.cpp:27)
main(/Volumes/T7/Github/code-search/examples/main.go:51)
main(/Volumes/T7/Github/code-search/examples/Main.java:31)
main(/Volumes/T7/Github/code-search/examples/main.py:48)
main(/Volumes/T7/Github/code-search/src/main.rs:32)

>> outline /Volumes/T7/Github/code-search/examples/main.go
type Shape interface
  Area
  Perimeter
type Rectangle struct
  Width
  Height
func (r Rectangle) Area () float64
func (r Rectangle) Perimeter () float64
```
