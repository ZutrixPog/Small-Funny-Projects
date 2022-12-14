use std::rc::Rc;

pub struct b_node {
    nodes: Vec<Rc<b_node>>,
    keys: Vec<i32>,
    t: i32,
    n: i32,
    leaf: bool
}

impl b_node {
    pub fn new_node(t: i32, leaf: bool) -> Rc<b_node> {
        Rc::new(b_node {
            nodes: Vec::with_capacity((2*t) as usize),
            keys: Vec::with_capacity((2*t-1) as usize),
            t,
            n: 0,
            leaf: false
        })
    }

    fn insert_non_full(&mut self, k: i32) {
        let mut i = self.n-1;

        if self.leaf == true {
            while i >= 0 && self.keys[i as usize] > k {
                self.keys[(i+1) as usize] = self.keys[i as usize];
                i -= 1;
            }

            self.keys[(i+1) as usize] = k;
            self.n = self.n+1;
        } else {
            while i >= 0 && self.keys[i as usize] > k {
                i -= 1;
            }

            if self.nodes[(i + 1) as usize].n == 2*self.t-1 {
                self.split_child((i+1) as i32, Rc::clone(&self.nodes[(i+1) as usize]));

                if self.keys[(i+1) as usize] < k {
                    i += 1;
                }
            }
            self.nodes[(i+1) as usize].insert_non_full(k);        
        }
    }

    fn split_child(&mut self, i: i32, mut y: Rc<b_node>) {
        let mut z = b_node::new_node(y.t, y.leaf);
        let mutref = Rc::get_mut(&mut z).unwrap();
        mutref.n = self.t -1;

        for j in 0..self.t-1 {
            mutref.keys[j as usize] = y.keys[(j+self.t) as usize];
        }

        if y.leaf == false {
            for j in 0..self.t {
                mutref.nodes[j as usize] = Rc::clone(&y.nodes[(j+self.t) as usize]);
            }
        }

        Rc::get_mut(&mut y).unwrap().n = self.t -1;

        for j in i+1..self.n {
            self.nodes[(j+1) as usize] = Rc::clone(&self.nodes[j as usize]);
        }

        self.nodes[(i+1) as usize] = z;

        for j in i..self.n -1 {
            self.keys[(j+1) as usize] = self.keys[j as usize];
        }

        self.keys[i as usize] = y.keys[(self.t-1) as usize];
        self.n = self.n + 1;
    }

    pub fn traverse(&self) {
        for (i, node) in self.nodes.iter().enumerate() {
            if self.leaf == false {
                node.traverse();
            }
            print!(" {}", self.keys[i as usize]);
        }

        if self.leaf == false {
            if let Some(last) = self.nodes.last() {
                last.traverse();
            }
        }
    }

    pub fn search(&self, k: i32) -> Option<Rc<b_node>> {
        let mut i:i32 = 0;
        while i < self.n && k > self.keys[i as usize] {
            i += 1;
        }

        if self.keys[i as usize] == k {
            return Some(Rc::new(&self));
        }

        if self.leaf == true {
            return None;
        }

        self.nodes[i as usize].search(k)
    }
}

pub struct b_tree {
    root: Option<Rc<b_node>>,
    t: i32
}

impl b_tree {
    pub fn new_tree(t: i32) -> b_tree {
        b_tree {
            root: None,
            t
        }
    }

    pub fn traverse(&self) {
        if let Some(root) = &self.root {
            root.traverse();
        }
    }

    pub fn search(&self, k: i32) -> Option<Rc<b_node>> {
        if let Some(root) = &self.root {
            root.search(k)
        } else {
            None
        }
    }

    pub fn insert(&mut self, k: i32) {
        if let Some(root) = &self.root {
            if self.root.as_ref().unwrap().n == 2*self.t-1 {
                let node = b_node::new_node(self.t, false);
                let mutnode = Rc::get_mut(&mut node).unwrap();
                mutnode.nodes[0] = Rc::clone(&root);
                mutnode.split_child(0, root);

                let mut i = 0;
                if node.keys[0] < k {
                    i += 1;
                }
                node.nodes[i as usize].insert_non_full(k);

                self.root = Some(node);
            } else {
                self.root.as_ref().unwrap().insert_non_full(k);
            }
        } else {
            self.root = Some(b_node::new_node(self.t, true));
            self.root.as_ref().unwrap().keys[0] = k;
            self.root.as_ref().unwrap().n = 1;
        }
    }
}