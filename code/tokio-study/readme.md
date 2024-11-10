https://rust-book.junmajinlong.com/ch100/00.html


runtime

异步运行时，异步任务要在runtime内才能使用

task

每定义一个Future(例如一个async语句块就是一个Future)，就定义了一个静止的尚未执行的task，当它在runtime中开始运行的时候，它就是真正的task，一个真正的异步任务。