use std::collections::HashMap;

pub fn learn_hashmap() {
    let mut games = HashMap::new();
    games.insert("RA2", "红色警戒: Red Alert 2");
    games.insert("Call Of Duty 6", "使命召唤6：现代战争2");

    for (k, v) in &games {
        println!("{}: {}", k, v);
    }

    let score_source = vec![
        ("A班", 89),
        ("B班", 96),
        ("C班", 83)
    ];

    // collect() 是个大聪明，会自己把各种类型转换为另一个类型，但需要指定要转移的目标类型，比如这里是 HashMap<_, _>
    // <_, _> 意思是让编译器自己推导个类型用，当然也可以自己指定，比如 <_, u8>，或 <&str, u8>
    // 注意此时 source_score 里的值所有权都转移进 scores 里了，不能再使用，除非实现了 Copy
    let scores: HashMap<_, _> = score_source.into_iter().collect();
    println!("{:?}", scores);
    println!("{}", scores.get("B班").unwrap_or(&0));

    let text = "hello world wonderful world";
    let mut words_count = HashMap::new();

    for word in text.split_whitespace() {
        //很神奇吧，函数或方法还可以返回一个 &mut 类型的值，此时就不用声明 let &mut count 了
        let count = words_count.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", words_count);
}