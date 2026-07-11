#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
        }
    }

    pub fn push(&mut self, value: T) {
        if let Some(node) = self.head.take() {
            let new_node = Node {
                value: value,
                next: Some(Box::new(node)),
            };
            self.head = Some(new_node)
        } else {
            let node = Node {
                value: value,
                next: None,
            };
            self.head = Some(node)
        }
    }

    pub fn pop(&mut self) {
        if let Some(node) = self.head.take() {
            if let Some(next_node) = node.next {
                let next = *next_node;
                self.head = Some(next)
            }
        } else {
            self.head = None
        }
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            count += 1;
            current = node.next.as_deref()
        }
        count
    }
}