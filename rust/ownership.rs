fn main() {
    //所有权（ownership）是 Rust 用于如何管理内存的一组规则。 -- 批注：这里我理解是管理堆上的内存
    //所有程序都必须管理其运行时使用计算机内存的方式。
    //一些语言中具有垃圾回收机制，在程序运行时有规律地寻找不再使用的内存；
    //在另一些语言中，程序员必须亲自分配和释放内存。
    //Rust 则选择了第三种方式：通过所有权系统管理内存，编译器在编译时会根据一系列的规则进行检查。
    //如果违反了任何这些规则，程序都不能编译。

    //所有权规则    --      批注：有点想cpp的uniqur_ptr
    //1.Rust 中的每一个值都有一个 所有者（owner）。
    //2.值在任一时刻有且只有一个所有者。
    //3.当所有者（变量）离开作用域，这个值将被丢弃。
    //      当变量离开作用域，Rust 为我们调用一个特殊的函数。这个函数叫做 drop。
    //      注意：在 C++ 中，这种 item 在生命周期结束时释放资源的模式有时被称作 资源获取即初始化（Resource Acquisition Is Initialization (RAII)）。如果你使用过 RAII 模式的话应该对 Rust 的 drop 函数并不陌生。
    //            C++通过在对象生命周期结束的时候通过析构函数释放资源
    //      浅拷贝（shallow copy）和 深拷贝（deep copy）
    //          Rust 永远也不会自动创建数据的 “深拷贝”。-- 批注：意味着牵拷贝，只拷贝指针
    //          因此，任何 自动 的复制都可以被认为是对运行时性能影响较小的。
    //          除非克隆    variable.clone()


    //只在栈上的数据：拷贝
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");
    //但这段代码似乎与我们刚刚学到的内容相矛盾：没有调用 clone，不过 x 依然有效且没有被移动到 y 中。
    //原因是像整型这样的在编译时已知大小的类型被整个存储在栈上，所以拷贝其实际的值是快速的。
    //这意味着没有理由在创建变量 y 后使 x 无效。
    //换句话说，这里没有深浅拷贝的区别，所以这里调用 clone 并不会与通常的浅拷贝有什么不同，我们可以不用管它。
    //Rust 有一个叫做 Copy trait 的特殊注解，可以用在类似整型这样的存储在栈上的类型上（第十章将会详细讲解 trait）。
    //  如果一个类型实现了 Copy trait，那么一个旧的变量在将其赋值给其他变量后仍然可用。
    //Rust 不允许自身或其任何部分实现了 Drop trait 的类型使用 Copy trait。
    //  如果我们对其值离开作用域时需要特殊处理的类型使用 Copy 注解，将会出现一个编译时错误。
    //  要学习如何为你的类型添加 Copy 注解以实现该 trait，请阅读附录 C 中的 “可派生的 trait”。
    //那么哪些类型实现了 Copy trait 呢？
    //  所有整数类型，比如 u32。
    //  布尔类型，bool，它的值是 true 和 false。
    //  所有浮点数类型，比如 f64。
    //  字符类型，char。
    //  元组，当且仅当其包含的类型也都实现 Copy 的时候。比如，(i32, i32) 实现了 Copy，但 (i32, String) 就没有。


    //引用（references）
    //  &variable
    //引用（reference）像一个指针，因为它是一个地址，我们可以由此访问储存于该地址的属于其他变量的数据。 
    //与指针不同，引用确保指向某个特定类型的有效值。
    //与使用 & 引用相反的操作是 解引用（dereferencing），它使用解引用运算符，*。我们将会在第八章遇到一些解引用运算符，并在第十五章详细讨论解引用。
    let s = String::from("hello");
    let s_r = &s; //不可变

    //mutable reference
    let mut s = String::from("hello");
    let mut s_r = &s;

    //引用的规则
    //  在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
    //      新 Rustacean 们经常难以适应这一点，因为大部分语言中变量任何时候都是可变的。
    //      这个限制的好处是 Rust 可以在编译时就避免数据竞争。
    //      数据竞争（data race）类似于竞态条件。
    //      数据竞争会导致未定义行为，难以在运行时追踪，并且难以诊断和修复；
    //      Rust 避免了这种情况的发生，因为它甚至不会编译存在数据竞争的代码！
    //
    //      不可以
    //          let mut s = String::from("hello");
    //          let r1 = &mut s;
    //          let r2 = &mut s;
    //          println!("{}, {}", r1, r2);
    //
    //          Rust 在同时使用可变与不可变引用时也采用的类似的规则。
    //          let mut s = String::from("hello");
    //          let r1 = &s; // 没问题
    //          let r2 = &s; // 没问题
    //          let r3 = &mut s; // 大问题
    //          println!("{}, {}, and {}", r1, r2, r3);
    //          哇哦！我们 也 不能在拥有不可变引用的同时拥有可变引用。 
    //          不可变引用的用户可不希望在他们的眼皮底下值就被意外的改变了！
    //          然而，多个不可变引用是可以的，因为没有哪个只能读取数据的人有能力影响其他人读取到的数据。
    //
    //      可以
    //          注意一个引用的作用域从声明的地方开始一直持续到最后一次使用为止。!!!
    //          例如，因为最后一次使用不可变引用（println!)，发生在声明可变引用之前，所以如下代码是可以编译的:
    //
    //          let mut s = String::from("hello");
    //          let r1 = &mut s;
    //          println!("{}", r1);
    //          let r2 = &mut s;
    //          println!("{}", r2);
    //
    //          let mut s = String::from("hello");
    //          {
    //              let r1 = &mut s;
    //          } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用
    //          let r2 = &mut s;
    //
    //          let mut s = String::from("hello");
    //          let r1 = &s; // 没问题
    //          let r2 = &s; // 没问题
    //          println!("{r1} and {r2}");
    //          // 此位置之后 r1 和 r2 不再使用
    //          let r3 = &mut s; // 没问题
    //          println!("{r3}");
    //          不可变引用 r1 和 r2 的作用域在 println! 最后一次使用之后结束，这也是创建可变引用 r3 的地方。
    //          它们的作用域没有重叠，所以代码是可以编译的。
    //          编译器可以在作用域结束之前判断不再使用的引用。
    //
    //
    //          尽管这些错误有时使人沮丧，但请牢记这是 Rust 编译器在提前指出一个潜在的 bug（在编译时而不是在运行时）并精准显示问题所在。
    //          这样你就不必去跟踪为何数据并不是你想象中的那样。
    //
    //  引用必须总是有效的。
    //      在具有指针的语言中，很容易通过释放内存时保留指向它的指针而错误地生成一个 悬垂指针（dangling pointer），所谓悬垂指针是其指向的内存可能已经被分配给其它持有者。
    //      相比之下，在 Rust 中编译器确保引用永远也不会变成悬垂状态：当你拥有一些数据的引用，编译器确保数据不会在其引用之前离开作用域。 
    //
    //      让我们尝试创建一个悬垂引用，Rust 会通过一个编译时错误来避免：
    //      fn main() {
    //          let reference_to_nothing = dangle();
    //      }
    //    
    //      fn dangle() -> &String {    // dangle 返回一个字符串的引用
    //          let s = String::from("hello");  // s 是一个新字符串
    //      
    //          &s  // 返回字符串 s 的引用
    //      }   // 这里 s 离开作用域并被丢弃。其内存被释放。
      
      
      
    //Slice
    //slice 允许你引用集合中一段连续的元素序列，而不用引用整个集合。
    //slice 是一种引用，所以它没有所有权。
    //字符串 slice（string slice）是 String 中一部分值的引用，它看起来像这样：
    //  let s = String::from("hello world");
    //  let hello = &s[0..5];
    //  let world = &s[6..11];
    //  可以使用一个由中括号中的 [starting_index..ending_index] 指定的 range 创建一个 slice，其中 starting_index 是 slice 的第一个位置，ending_index 则是 slice 最后一个位置的后一个值。
    //  在其内部，slice 的数据结构存储了 slice 的开始位置和长度，长度对应于 ending_index 减去 starting_index 的值。

    //  对于 Rust 的 .. range 语法，如果想要从索引 0 开始，可以不写两个点号之前的值。
    let s = String::from("hello");

    let slice = &s[0..2];
    let slice = &s[..2]
    //依此类推，如果 slice 包含 String 的最后一个字节，也可以舍弃尾部的数字。
    let s = String::from("hello");

    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];

    //也可以同时舍弃这两个值来获取整个字符串的 slice。
    let s = String::from("hello");

    let len = s.len();

    let slice = &s[0..len];
    let slice = &s[..];

    //编译器会确保指向 String 的引用持续有效
    //错误
    //  let mut s = String::from("hello world");
    //  let word = first_word(&s);
    //  s.clear(); // 错误！
    //  println!("the first word is: {word}");

    //回忆一下借用规则，当拥有某值的不可变引用时，就不能再获取一个可变引用。
    //因为 clear 需要清空 String，它尝试获取一个可变引用。
    //在调用 clear 之后的 println! 使用了 word 中的引用，所以这个不可变的引用在此时必须仍然有效。
    //Rust 不允许 clear 中的可变引用和 word 中的不可变引用同时存在，因此编译失败。
    //Rust 不仅使得我们的 API 简单易用，也在编译时就消除了一整类的错误！

    //字符串字面值就是 slice
    //字符串字面值被储存在二进制文件中
    let s = "Hello, world!";
    //这里 s 的类型是 &str：它是一个指向二进制程序特定位置的 slice。
    //这也就是为什么字符串字面值是不可变的；&str 是一个不可变引用。

    //字符串 slice 作为参数
    //而更有经验的 Rustacean 会编写出下面的签名，因为它使得可以对 &String 值和 &str 值使用相同的函数：
    //fn first_word(s: &str) -> &str {
    //如果有一个字符串 slice，可以直接传递它。
    //如果有一个 String，则可以传递整个 String 的 slice 或对 String 的引用。
    //  这种灵活性利用了 deref coercions 的优势，这个特性我们将在“函数和方法的隐式 Deref 强制转换”章节中介绍。
    //  定义一个获取字符串 slice 而不是 String 引用的函数使得我们的 API 更加通用并且不会丢失任何功能。

    //其他类型的 slice
    //字符串 slice，正如你想象的那样，是针对字符串的。不过也有更通用的 slice 类型。
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);

}

//“字符串 slice” 的类型声明写作 &str
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
