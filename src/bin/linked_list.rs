pub struct LinkedList<T>
where
    T: Copy,
{
    first: *mut Node<T>,
    len: usize,
}

#[allow(dead_code)]
impl<T> LinkedList<T>
where
    T: Copy,
{
    pub fn new() -> LinkedList<T> {
        LinkedList {
            first: std::ptr::null_mut(),
            len: 0,
        }
    }
    pub fn first(&mut self) -> LinkedListIndex<T> {
        LinkedListIndex::new(self, self.first)
    }
    pub fn last(&mut self) -> LinkedListIndex<T> {
        LinkedListIndex::new(self, Node::previous(self.first))
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<T> Drop for LinkedList<T>
where
    T: Copy,
{
    fn drop(&mut self) {
        unsafe {
            let mut current = self.first;
            let mut next;
            for _ in 0..self.len {
                next = current.as_mut().unwrap().next;
                Box::from_raw(current);
                current = next;
            }
        }
    }
}

pub struct LinkedListIndex<T>
where
    T: Copy,
{
    list: *mut LinkedList<T>,
    node: *mut Node<T>,
}

#[allow(dead_code)]
impl<T> LinkedListIndex<T>
where
    T: Copy,
{
    fn new(list: *mut LinkedList<T>, node: *mut Node<T>) -> LinkedListIndex<T> {
        LinkedListIndex { list, node }
    }
    pub fn insert_after(&mut self, data: T) {
        let list = unsafe { self.list.as_mut().unwrap() };
        self.node = Node::insert_after(self.node, data);
        if list.is_empty() {
            list.first = self.node;
        }
        list.len += 1;
    }
    pub fn insert_before(&mut self, data: T) {
        let list = unsafe { self.list.as_mut().unwrap() };
        self.node = Node::insert_before(self.node, data);
        if list.is_empty() {
            list.first = self.node;
        }
        list.len += 1;
    }
    pub fn remove(&mut self) -> T {
        let list = unsafe { self.list.as_mut().unwrap() };
        if list.len > 0 {
            let data = self.data();
            let was_first = self.node == list.first;
            self.node = Node::remove(self.node);
            list.len -= 1;
            if was_first {
                list.first = self.node;
            }
            data
        } else {
            panic!("Called remove on empty linked_list")
        }
    }
    pub fn data(&self) -> T {
        Node::data(self.node)
    }
}

impl<T> Iterator for LinkedListIndex<T>
where
    T: Copy,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let data = Node::data(self.node);
        self.node = Node::next(self.node);
        Some(data)
    }
}

impl<T> DoubleEndedIterator for LinkedListIndex<T>
where
    T: Copy,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let data = Node::data(self.node);
        self.node = Node::previous(self.node);
        Some(data)
    }
}

struct Node<T>
where
    T: Copy,
{
    data: T,
    previous: *mut Node<T>,
    next: *mut Node<T>,
}

impl<T> Node<T>
where
    T: Copy,
{
    fn new(data: T) -> *mut Node<T> {
        unsafe {
            let ll = Box::into_raw(Box::new(Node {
                data,
                previous: 0 as *mut Node<T>,
                next: 0 as *mut Node<T>,
            }));
            ll.as_mut().unwrap().previous = ll;
            ll.as_mut().unwrap().next = ll;
            ll
        }
    }

    fn data(index: *mut Node<T>) -> T {
        unsafe {
            if let Some(node) = index.as_ref() {
                node.data
            } else {
                panic!("Dereferenced null pointer in linked list");
            }
        }
    }

    fn previous(index: *mut Node<T>) -> *mut Node<T> {
        unsafe { index.as_ref().unwrap().previous }
    }

    fn next(index: *mut Node<T>) -> *mut Node<T> {
        unsafe { index.as_ref().unwrap().next }
    }

    fn insert_after(index: *mut Node<T>, data: T) -> *mut Node<T> {
        unsafe {
            let node;
            if index.is_null() {
                node = Node::new(data)
            } else {
                let next = index.as_ref().unwrap().next;
                node = Box::into_raw(Box::new(Node {
                    data,
                    previous: index,
                    next,
                }));
                index.as_mut().unwrap().next = node;
                next.as_mut().unwrap().previous = node;
            }
            node
        }
    }

    fn insert_before(index: *mut Node<T>, data: T) -> *mut Node<T> {
        unsafe {
            let node;
            if index.is_null() {
                node = Node::new(data)
            } else {
                let previous = index.as_ref().unwrap().previous;
                node = Box::into_raw(Box::new(Node {
                    data,
                    previous,
                    next: index,
                }));
                previous.as_mut().unwrap().next = node;
                index.as_mut().unwrap().previous = node;
            }
            node
        }
    }

    fn remove(index: *mut Node<T>) -> *mut Node<T> {
        unsafe {
            let previous = index.as_ref().unwrap().previous;
            let mut next = index.as_ref().unwrap().next;
            if index != next {
                previous.as_mut().unwrap().next = next;
                next.as_mut().unwrap().previous = previous;
            } else {
                next = std::ptr::null_mut();
            }
            Box::from_raw(index);
            next
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut ll = LinkedList::new();
        let mut index = ll.first();
        index.insert_after(0);
        index.insert_after(1);
        index.insert_after(2);
        index.insert_after(6);
        index.insert_after(3);
        index.insert_after(5);
        index.insert_before(4);
        index.nth_back(1).unwrap();
        index.remove();
        index = ll.last();
        index.insert_after(6);
        assert_eq!(index.next(), Some(6));
        index.remove();
        let len = ll.len();
        assert_eq!(len, 6);
        index = ll.first();
        let items: Vec<_> = (0..len).map(|_| index.next().unwrap()).collect();
        assert_eq!(items, vec![1, 2, 3, 4, 5, 6]);
    }
}

#[allow(dead_code)]
fn main() {
    unimplemented!()
}
