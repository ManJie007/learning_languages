//定义并实例化结构体
//和元组一样，结构体的每一部分可以是不同类型。
//但不同于元组，结构体需要命名各部分数据以便能清楚的表明其值的意义。
//由于有了这些名字，结构体比元组更灵活：不需要依赖顺序来指定或访问实例中的值。
//struct struct_name { field_name:field_type,... }

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//User 结构体的定义中，我们使用了自身拥有所有权的 String 类型而不是 &str 字符串 slice 类型。
//这是一个有意而为之的选择，因为我们想要这个结构体拥有它所有的数据，为此只要整个结构体是有效的话其数据也是有效的。
//可以使结构体存储被其他对象拥有的数据的引用，不过这么做的话需要用上生命周期（lifetime），这是一个第 10 章会讨论的 Rust 功能。
//生命周期确保结构体引用的数据有效性跟结构体本身保持一致。如果你尝试在结构体中存储一个引用而不指定生命周期将是无效的
//比如：
//      struct User {
//          active: bool,
//          username: &str,
//          email: &str,
//          sign_in_count: u64,
//      }
//      
//      fn main() {
//          let user1 = User {
//              email: "someone@example.com",
//              username: "someusername123",
//              active: true,
//              sign_in_count: 1,
//          };
//      }
//编译器会抱怨它需要生命周期标识符

//使用没有命名字段的元组结构体来创建不同的类型
//也可以定义与元组（在第 3 章讨论过）类似的结构体，称为元组结构体（tuple struct）。
//  元组结构体有着结构体名称提供的含义，但没有具体的字段名，只有字段的类型。
//  当你想给整个元组取一个名字，并使元组成为与其他元组不同的类型时，元组结构体是很有用的，这时像常规结构体那样为每个字段命名就显得多余和形式化了。
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//没有任何字段的类单元结构体
//它们被称为类单元结构体（unit-like structs），因为它们类似于 ()，即“元组类型”一节中提到的 unit 类型。
//类单元结构体常常在你想要在某个类型上实现 trait 但不需要在类型中存储数据的时候发挥作用。
//不需要花括号或圆括号！
//想象一下，我们将实现这个类型的行为，即每个实例始终等于每一个其他类型的实例，也许是为了获得一个已知的结果以便进行测试。
//我们不需要任何数据来实现这种行为，你将在第十章中，看到如何定义特性并在任何类型上实现它们，包括类单元结构体。
struct AlwaysEqual;

fn main() {
//要在定义结构体后使用它，我们可以通过为每个字段指定具体值的方式来创建该结构体的实例
//实例中字段的顺序不需要和它们在结构体中声明的顺序一致。
//换句话说，结构体的定义就像一个类型的通用模板，而实例则会在这个模板中放入特定数据来创建这个类型的值。
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
//为了从结构体中获取某个特定的值，可以使用点号。
//要更改结构体中的值，如果结构体的实例是可变的，我们可以使用点号并为对应的字段赋值。
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
//注意整个实例必须是可变的；Rust 并不允许只将某个字段标记为可变。


//使用结构体更新语法从其他实例创建实例
//使用旧实例的大部分值但改变其部分值来创建一个新的结构体实例通常很有用。
//这可以通过结构体更新语法（struct update syntax）实现。
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

//使用结构体更新语法，.. 语法指定了剩余未显式设置值的字段应有与给定实例对应字段相同的值。
    let user2 = User {
        email: String::from("another@example.com"),
//..user1 必须放在最后，以指定其余的字段应从 user1 的相应字段中获取其值，但我们可以选择以任何顺序为任意字段指定值，而不用考虑结构体定义中字段的顺序。
        ..user1
    };

//请注意，结构更新语法就像带有 = 的赋值，因为它移动了数据，就像我们在./ownership.rs 移动部分讲到的一样。
//在这个例子中，我们在创建 user2 后不能再使用 user1，因为 user1 的 username 字段中的 String 被移到 user2 中。
//如果我们给 user2 的 email 和 username 都赋予新的 String 值，从而只使用 user1 的 active 和 sign_in_count 值，那么 user1 在创建 user2 后仍然有效。
//active 和 sign_in_count 的类型是实现 Copy trait 的类型，所以我们在./ownership.rs 克隆部分讨论的行为同样适用。

//你定义的每一个结构体有其自己的类型，即使结构体中的字段有着相同的类型。
//  例如，一个获取 Color 类型参数的函数不能接受 Point 作为参数，即便这两个类型都由三个 i32 值组成。
//在其他方面，元组结构体实例类似于元组：可以将其解构为单独的部分，也可以使用 . 后跟索引来访问单独的值，等等。
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

//一个声明和实例化一个名为 AlwaysEqual 的 unit 结构的例子。
//不需要任何花括号或圆括号
    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
//另外需要注意同其他任何表达式一样，我们可以在函数体的最后一个表达式中构造一个结构体的新实例，来隐式地返回这个实例。
//为函数参数起与结构体字段相同的名字是可以理解的，但是不得不重复 email 和 username 字段名称与变量有些啰嗦。如果结构体有更多字段，重复每个名称就更加烦人了。
//幸运的是，有一个方便的简写语法！
//  字段初始化简写语法（field init shorthand）来重写 build_user，这样其行为与之前完全相同
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

//字段初始化简写语法（field init shorthand）来重写 build_user，这样其行为与之前完全相同
fn build_user_field_init_shorthand(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

//一个使用结构体的示例程序
//cargo new rectangles
//
//main.rs
//
//      fn main() {
//          let width1 = 30;
//          let height1 = 50;
//      
//          println!(
//              "The area of the rectangle is {} square pixels.",
//              area(width1, height1)
//          );
//      }
//      
//      fn area(width: u32, height: u32) -> u32 {
//          width * height
//      }
//
//cargo run
//
//宽度和高度是相关联的，因为他们在一起才能定义一个长方形。
//函数 area 本应该计算一个长方形的面积，不过函数却有两个参数。这两个参数是相关联的，不过程序本身却没有表现出这一点。
//将长度和宽度组合在一起将更易懂也更易处理。
//
//使用元组重构
//main.rs
//
//      fn main() {
//          let rect1 = (30, 50);
//      
//          println!(
//              "The area of the rectangle is {} square pixels.",
//              area(rect1)
//          );
//      }
//      
//      fn area(dimensions: (u32, u32)) -> u32 {
//          dimensions.0 * dimensions.1
//      }
//
//元组帮助我们增加了一些结构性，并且现在只需传一个参数。
//不过在另一方面，这个版本却有一点不明确了：元组并没有给出元素的名称，所以我们不得不使用索引来获取元组的每一部分，这导致计算变得更令人费解了。
//
//
//使用结构体重构：赋予更多意义
//
//main.rs
//
//      struct Rectangle {
//          width: u32,
//          height: u32,
//      }
//      
//      fn main() {
//          let rect1 = Rectangle {
//              width: 30,
//              height: 50,
//          };
//      
//          println!(
//              "The area of the rectangle is {} square pixels.",
//              area(&rect1)
//          );
//      }
//      
//      fn area(rectangle: &Rectangle) -> u32 {
//          rectangle.width * rectangle.height
//      }
//
//们希望借用结构体而不是获取它的所有权，这样 main 函数就可以保持 rect1 的所有权并继续使用它，所以这就是为什么在函数签名和调用的地方会有 &。
//使用 Rectangle 的 width 和 height 字段，计算 Rectangle 的面积。
//这表明宽高是相互联系的，并为这些值提供了描述性的名称而不是使用元组的索引值 0 和 1 。结构体胜在更清晰明了。

//通过派生 trait 增加实用功能
//如果能够在调试程序时打印出 Rectangle 实例来查看其所有字段的值就更好了。
//像前面章节那样尝试使用 println! 宏。但这并不行。
//
//main.rs
//
//      struct Rectangle {
//          width: u32,
//          height: u32,
//      }
//      
//      fn main() {
//          let rect1 = Rectangle {
//              width: 30,
//              height: 50,
//          };
//      
//          println!("rect1 is {}", rect1);
//      }
//
//error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
//
//println! 宏能处理很多类型的格式，不过，{} 默认告诉 println! 使用被称为 Display 的格式：意在提供给直接终端用户查看的输出。
//目前为止见过的基本类型都默认实现了 Display，因为它就是向用户展示或其他任何基本类型的唯一方式。
//不过对于结构体，println! 应该用来输出的格式是不明确的，因为这有更多显示的可能性：是否需要逗号？需要打印出大括号吗？所有字段都应该显示吗？由于这种不确定性，Rust 不会尝试猜测我们的意图，所以结构体并没有提供一个 Display 实现。
//
//= help: the trait `std::fmt::Display` is not implemented for `Rectangle`
//= note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
//
//在 {} 中加入 :? 指示符告诉 println! 我们想要使用叫做 Debug 的输出格式。
//Debug 是一个 trait，它允许我们以一种对开发者有帮助的方式打印结构体，以便当我们调试代码时能看到它的值。
//
//这样调整后再次运行程序。
//
//error[E0277]: `Rectangle` doesn't implement `Debug`
//
//= help: the trait `Debug` is not implemented for `Rectangle`
//= note: add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`
//
//Rust 确实包含了打印出调试信息的功能，不过我们必须为结构体显式选择这个功能。
//为此，在结构体定义之前加上外部属性 #[derive(Debug)]
//增加属性来派生 Debug trait，并使用调试格式打印 Rectangle 实例
//
//main.rs
//
//      #[derive(Debug)]
//      struct Rectangle {
//          width: u32,
//          height: u32,
//      }
//      
//      fn main() {
//          let rect1 = Rectangle {
//              width: 30,
//              height: 50,
//          };
//      
//          println!("rect1 is {:?}", rect1);
//      }
//
//cargo run
//
//当我们有一个更大的结构体时，能有更易读一点的输出就好了，为此可以使用 {:#?} 替换 println! 字符串中的 {:?}。
//
//
//另一种使用 Debug 格式打印数值的方法是使用 dbg! 宏。
//dbg! 宏接收一个表达式的所有权，打印出代码中调用 dbg! 宏时所在的文件和行号，以及该表达式的结果值，并返回该值的所有权。
//注释：与 println! 宏打印到标准输出控制流（stdout）不同，调用 dbg! 宏会打印到标准错误控制流（stderr）。
//  我们将在第 12 章 “将错误信息输出到标准错误而不是标准输出” 一节中更多地讨论 stderr 和 stdout。
//
//  main.rs
//
//          #[derive(Debug)]
//          struct Rectangle {
//              width: u32,
//              height: u32,
//          }
//          
//          fn main() {
//              let scale = 2;
//              let rect1 = Rectangle {
//                  width: dbg!(30 * scale),
//                  height: 50,
//              };
//          
//              dbg!(&rect1);
//          }
//
//我们可以把 dbg! 放在表达式 30 * scale 周围，因为 dbg! 返回表达式的值的所有权，所以 width 字段将获得相同的值，就像我们在那里没有 dbg! 调用一样。
//我们不希望 dbg! 拥有 rect1 的所有权，所以我们在下一次调用 dbg! 时传入一个引用。
//$ cargo run
//   Compiling rectangles v0.1.0 (file:///projects/rectangles)
//    Finished dev [unoptimized + debuginfo] target(s) in 0.61s
//     Running `target/debug/rectangles`
//[src/main.rs:10] 30 * scale = 60
//[src/main.rs:14] &rect1 = Rectangle {
//    width: 60,
//    height: 50,
//}
//
//除了 Debug trait，Rust 还为我们提供了很多可以通过 derive 属性来使用的 trait，他们可以为我们的自定义类型增加实用的行为。
//附录 C 中列出了这些 trait 和行为。第十章会介绍如何通过自定义行为来实现这些 trait，同时还有如何创建你自己的 trait。
//除了 derive 之外，还有很多属性；更多信息请参见 《Rust 参考手册》 的 Attributes 部分。
//
//我们的 area 函数非常特殊，它只计算长方形的面积。
//如果这个行为与 Rectangle 结构体再结合得更紧密一些就更好了，因为它不能用于其他类型。
  
  
//方法语法
//方法与函数类似：它们使用 fn 关键字和名称声明，可以拥有参数和返回值，同时包含在某处调用该方法时会执行的代码。
//不过方法与函数是不同的，因为它们在结构体的上下文中被定义（或者是枚举或 trait 对象的上下文，将分别在第 6 章和第 17 章讲解），并且它们第一个参数总是 self，它代表调用该方法的结构体实例。

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//把前面实现的获取一个 Rectangle 实例作为参数的 area 函数，改写成一个定义于 Rectangle 结构体上的 area 方法
impl Rectangle {
//&self 实际上是 self: &Self 的缩写。在一个 impl 块中，Self 类型是 impl 块的类型的别名。
//方法的第一个参数必须有一个名为 self 的Self 类型的参数，所以 Rust 让你在第一个参数位置上只用 self 这个名字来缩写。
//注意，我们仍然需要在 self 前面使用 & 来表示这个方法借用了 Self 实例，就像我们在 rectangle: &Rectangle 中做的那样。
//方法可以选择获得 self 的所有权，或者像我们这里一样不可变地借用 self，或者可变地借用 self，就跟其他参数一样。
//如果想要在方法中改变调用方法的实例，需要将第一个参数改为 &mut self。
//通过仅仅使用 self 作为第一个参数来使方法获取实例的所有权是很少见的；这种技术通常用在当方法将 self 转换成别的实例的时候，这时我们想要防止调用者在转换之后使用原始的实例。
  
//使用方法替代函数，除了可使用方法语法和不需要在每个函数签名中重复 self 的类型之外，其主要好处在于组织性。
//我们将某个类型实例能做的所有事情都一起放入 impl 块中，而不是让将来的用户在我们的库中到处寻找 Rectangle 的功能。
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

//请注意，我们可以选择将方法的名称与结构中的一个字段相同。
  
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}

//通常，但并不总是如此，与字段同名的方法将被定义为只返回字段中的值，而不做其他事情。
//这样的方法被称为 getters，Rust 并不像其他一些语言那样为结构字段自动实现它们。
//Getters 很有用，因为你可以把字段变成私有的，但方法是公共的，这样就可以把对字段的只读访问作为该类型公共 API 的一部分。
//我们将在第七章中讨论什么是公有和私有，以及如何将一个字段或方法指定为公有或私有。
  
  
//-> 运算符到哪去了？
//在 C/C++ 语言中，有两个不同的运算符来调用方法：. 直接在对象上调用方法，而 -> 在一个对象的指针上调用方法，这时需要先解引用（dereference）指针。
//      换句话说，如果 object 是一个指针，那么 object->something() 就像 (*object).something() 一样。
  
//Rust 并没有一个与 -> 等效的运算符；相反，Rust 有一个叫 自动引用和解引用（automatic referencing and dereferencing）的功能。
//      方法调用是 Rust 中少数几个拥有这种行为的地方。
  
//它是这样工作的：当使用 object.something() 调用方法时，Rust 会自动为 object 添加 &、&mut 或 * 以便使 object 与方法签名匹配。
//  也就是说，这些代码是等价的：
p1.distance(&p2);
(&p1).distance(&p2);
//第一行看起来简洁的多。这种自动引用的行为之所以有效，是因为方法有一个明确的接收者———— self 的类型。
//事实上，Rust 对方法接收者的隐式借用让所有权在实践中更友好。
  
  
//带有更多参数的方法
//通过观察调用方法的代码可以看出参数是什么类型的：rect1.can_hold(&rect2) 传入了 &rect2，它是一个 Rectangle 的实例 rect2 的不可变借用。
//这是可以理解的，因为我们只需要读取 rect2（而不是写入，这意味着我们需要一个不可变借用），而且希望 main 保持 rect2 的所有权，这样就可以在调用这个方法后继续使用它。
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

//关联函数
//所有在 impl 块中定义的函数被称为关联函数（associated function），因为它们与 impl 后面命名的类型相关。
//我们可以定义不以 self 为第一参数的关联函数（因此不是方法），因为它们并不作用于一个结构体的实例。
//我们已经使用了一个这样的函数，String::from 函数，它是在 String 类型上定义的。

impl Rectangle {
//关联函数经常被用作返回一个结构体新实例的构造函数。
//  例如我们可以提供一个关联函数，它接受一个维度参数并且同时作为宽和高，这样可以更轻松的创建一个正方形 Rectangle 而不必指定两次同样的值：
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

//使用结构体名和 :: 语法来调用这个关联函数：比如 let sq = Rectangle::square(3);。
//这个方法位于结构体的命名空间中：:: 语法用于关联函数和模块创建的命名空间。
//第 7 章会讲到模块。
  
  
//多个 impl 块
//每个结构体都允许拥有多个 impl 块。
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

//这里没有理由将这些方法分散在多个 impl 块中，不过这是有效的语法。
//第 10 章讨论泛型和 trait 时会看到实用的多 impl 块用例。
  
  
//结构体让你可以创建出在你的领域中有意义的自定义类型。
//  通过结构体，我们可以将相关联的数据片段联系起来并命名它们，这样可以使得代码更加清晰。
//  在 impl 块中，你可以定义与你的类型相关联的函数，而方法是一种相关联的函数，让你指定结构体的实例所具有的行为。
//  但结构体并不是创建自定义类型的唯一方法：让我们转向 Rust 的枚举功能，为你的工具箱再添一个工具。
