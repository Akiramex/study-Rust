## 并发

### 通道 mpsc 进行线程之间的通信

## 使用并发HashMap来实时收集信息

### 使用RwLock替换Mutex

读写锁可以降低性能开销

### 使用dashmap替换hashmap

dashmap内置了线程安全的设计，使得代码更加简洁，效率更高

### 用Atomic类型替换

Cpu对atomic类型有特殊的优化，使得操作atomic类型是原子性的，所以atomic类型是线程安全的