use std::ops::Add;
use super::structs::Point;

trait Draw {
    fn draw(&self) -> String;
}

impl Draw for Point {
    fn draw(&self) -> String {
        format!("Point: <x={}, y={}>", self.x, self.y)
    }
}

struct Button {
    pub width: u16,
    pub height: u16,
    pub label: String
}

impl Draw for Button {
    fn draw(&self) -> String {
        format!("Button: <\"{}\", {}x{}>", self.label, self.width, self.height)
    }
}

impl Draw for String {
    fn draw(&self) -> String {
        format!("String: \"{}\"", self)
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

trait Call {
    fn name() -> String;
}

impl Call for Point {
    fn name() -> String {
        String::from("Point")
    }
}

impl Call for Button {
    fn name() -> String {
        return String::from("Button")
    }
}

pub fn learn_trait_object() {
    //dyn 的意思是动态分发，特征只要求实现 Draw 而不要求 Vec 里全是同一种类型
    //这种存储的是不同类型，且在运行时才检查的即是动态分发
    let mut ui: Vec<Box<dyn Draw>> = Vec::new();

    let point = Point {
        x: 123,
        y: 321
    };

    let button = Button {
        width: 100,
        height: 80,
        label: String::from("Yes")
    };

    let str = String::from("With me");

    ui.push(Box::new(point));
    ui.push(Box::new(button));
    ui.push(Box::new(str));

    for r in ui.iter() {
        let ui_text = r.draw();
        println!("UI -> {}", ui_text);
    }

    let point_b = Point {
        x: 10,
        y: 10
    };

    let point_x = point + point_b;
    println!("{:?}", point_x);

    //直接调用目标类型，或使用完全限定语法
    println!("Point name: {}, {}", Point::name(), <Point as Call>::name());
    println!("Button name: {}, {}", Button::name(), <Button as Call>::name());

}
