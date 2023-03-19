struct Node<T> {
    url: T,
    next: Link<T>,
    prev: Link<T>
}

impl<T> Node<T> {
    fn new(val: T) -> *mut Self {
        Box::into_raw(Box::new(Self {
            url: val,
            next: std::ptr::null_mut(),
            prev: std::ptr::null_mut()
        }))
    }
}

type Link<T> = *mut Node<T>;

struct BrowserHistory {
    head: Link<String>,
    tail: Link<String>,
    curr: Link<String>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BrowserHistory {

    fn new(homepage: String) -> Self {
        let root = Node::new(homepage);
        Self {
            head: root,
            tail: root,
            curr: root,
        }
    }
    
    fn visit(&mut self, url: String) {
        unsafe {
            let new_url = Node::new(url);
            while self.curr != self.tail {
                let prev = (*self.tail).prev;
                Box::from_raw(self.tail);
                self.tail = prev;
            }
            (*new_url).prev = self.curr;
            (*self.curr).next = new_url;
            self.tail = new_url;

            self.curr = new_url;
        }
    }
    
    fn back(&mut self, steps: i32) -> String {
        unsafe {
            for i in (0..steps as usize) {
                if self.curr != self.head {
                    let prev = (*self.curr).prev;
                    self.curr = prev;
                } else {
                    break;
                }
            }
            (*self.curr).url.clone()
        }
    }
    
    fn forward(&mut self, steps: i32) -> String {
        unsafe {
            for i in (0..steps as usize) {
                if self.curr != self.tail {
                    let next = (*self.curr).next;
                    self.curr = next;
                } else {
                    break;
                }
            }
            (*self.curr).url.clone()
        }
    }
}

impl Drop for BrowserHistory {
    fn drop(&mut self) {
        unsafe {
            let mut node = self.head;
            while !node.is_null() {
                let next = (*node).next;
                Box::from_raw(node);
                node = next;
            }
        }
    }
}

/**
 * BrowserHistory object can be instantiated and called as such:
 * let obj = BrowserHistory::new(homepage);
 * obj.visit(url);
 * let ret_2: String = obj.back(steps);
 * let ret_3: String = obj.forward(steps);
 */
