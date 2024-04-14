pub fn sort_integer_vector() {
    let mut vec = vec![1, 5, 10, 2, 15];

    vec.sort();

    assert_eq!(vec, vec![1, 2, 5, 10, 15]);
}

pub fn sort_float_vector() {
    let mut vec = vec![1.1, 1.15, 5.5, 1.123, 2.0];

    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

    assert_eq!(vec, vec![1.1, 1.123, 1.15, 2.0, 5.5]);
}

// 引入 Debug, Eq, Ord, PartialEq, PartialOrd trait 以支持打印、比较和排序操作。
// Debug: 允许使用 {:?} 格式化标记打印结构体的信息。
// Eq 和 PartialEq: 允许结构体实例之间进行相等性比较。
// Ord 和 PartialOrd: 允许进行全排序，这是必须的，因为需要对结构体实例集合进行排序。
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    /// 创建并返回一个新的 Person 实例
    /// - 参数 `name`: 人员的名字
    /// - 参数 `age`: 人员的年龄
    pub fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }
}

pub fn sort_struct_vector() {
    // 初始化一个 Person 类型的向量，其中包含三个人员信息
    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];

    // 对 people 向量进行排序。排序基于 Person 结构体实现的 Ord trait。
    // 默认排序首先基于 name 字段的字典序，如果 name 相同，则比较 age 字段。
    people.sort();
    // 断言排序后的结果是否符合预期
    assert_eq!(
        people,
        vec![
            Person::new("Al".to_string(), 60),
            Person::new("John".to_string(), 1),
            Person::new("Zoe".to_string(), 25),
        ]
    );

    // 使用自定义的排序规则，通过 sort_by 方法指定一个闭包函数来比较 Person 实例。
    // 此例中按照 age 字段从大到小排序。
    people.sort_by(|a, b| b.age.cmp(&a.age));
    // 断言自定义排序后的结果是否符合预期
    assert_eq!(
        people,
        vec![
            Person::new("Al".to_string(), 60),
            Person::new("Zoe".to_string(), 25),
            Person::new("John".to_string(), 1),
        ]
    );
}
