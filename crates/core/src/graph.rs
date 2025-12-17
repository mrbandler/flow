struct Id {}

struct Position {
    line: u32,
    column: u32,
}

struct Node {
    id: Id,
    file: PathBuf,
    position: Position,
    content: String,
    children: Vec<Node>,
    parent: Option<Node>,
    tags: Vec<Node>,
    properties: HashMap<String, String>,
}

struct Graph {}
