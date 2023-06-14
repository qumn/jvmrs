# jvmrs
一个使用 **Rust** 编写的 **JVM**, 用于学习使用.

## 运行
1. 编写一个简单的 Java 程序 `GaussTest.java`
```java
public class GaussTest {
    public static void main(String[] args){
        int sum = 0;
        for (int i = 0; i <= 100; i++){
            sum += i;
        }
        System.out.println(sum);
    }
}
```

2. 编译 Java 文件为 Class
```shell
java GaussTest.java
```

3. 运行项目
```shell
cargo run -- -x /Users/qumn/Library/Java/JavaVirtualMachines/corretto-1.8.0_342/Contents/Home GaussTest
```
其中 `-x` 指定 `jre` 的位置, 替换为你的 `jre` 目录

## Futures
- [x] 类加载器   
- [x] 解析类文件   
- [x] 运行时数据区   
- [x] 指令和解释器
- [ ] 方法调用和返回
- [ ] 数组和字符
- [ ] 本体方法调用
- [ ] 异常处理
