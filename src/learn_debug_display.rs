use std::fmt::Display;

#[derive(Debug)]
enum FileState {
    Open,
    Closed
}

#[derive(Debug)]
struct File {
    name: String,
    data: String,
    state: FileState
}

impl Display for FileState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED")
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<name={}, state={}, data=\"{}\">", self.name, self.state, self.data)
    }
}

pub fn learn_debug_display() {
    let file = File {
        name: String::from("data.txt"),
        data: String::from("This is file data."),
        state: FileState::Closed
    };

    //实现了 Display 特性的，是要用 {} 才会调用 Display 来打印，而不是 {:?}
    println!("{}", file);
}
