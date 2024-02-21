#[derive(Clone, Debug)]
struct Rope {
    root: Node,
}

//https://github.com/Ben-Lichtman/editr/blob/master/src/rope.rs#L11
#[derive(Clone, Debug)]
enum Node {
    Leaf(LeafData),
    Internal(InternalData),
}

#[derive(Clone, Debug)]
struct LeafData {
    content: Vec<u8>,
}

#[derive(Clone, Debug)]
struct InternalData {
    index: usize,
    size: usize,
    children: Box<(Node, Node)>,
}

impl Rope {
    fn insert(&mut self, pos: usize, value: Vec<char>) {}
    fn rebalance(&self) -> Self {
        let data = self.collect();
        self.clone()
    }
    fn collect(&self) -> &str {
        ""
    }
}

impl From<Vec<char>> for Rope {
    fn from(value: Vec<char>) -> Self {
        let parent = Rope {
            root: Node::Leaf(LeafData {
                content: value.iter().map(|x| *x as u8).collect(),
            }),
        };
        parent.rebalance()
    }
}
impl From<&str> for Rope {
    fn from(value: &str) -> Self {
        let parent = Rope {
            root: Node::Leaf(LeafData {
                content: value.into(),
            }),
        };
        parent.rebalance()
    }
}
impl From<String> for Rope {
    fn from(value: String) -> Self {
        let parent = Rope {
            root: Node::Leaf(LeafData {
                content: value.into(),
            }),
        };
        parent.rebalance()
    }
}

pub fn main() {
    let mut test: Rope = "test".into();
    test.insert(12, vec!['a']);
}

fn calc_fib(n: u32) -> u32 {
    let mut nums: (u32, u32) = (0, 0);
    if n == 0 {
        return nums.0;
    }
    let mut c = 0;
    let mut i = 2;
    while i <= n {
        c = nums.0 + nums.1;
        nums.0 = nums.1;
        nums.1 = c;
        i += 1;
    }
    nums.1
}
