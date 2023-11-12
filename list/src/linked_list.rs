use std::sync::{Arc, Mutex};
// Rc::RefCell can also be used (non thread safe though)
pub type Link<T> = Arc<Mutex<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    item: T,
    previous: Option<Link<T>>,
    next: Option<Link<T>>,
}

impl<T> Node<T> {
    fn new(item: T) -> Self {
        Self {
            item,
            previous: None,
            next: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DoubleLinkedList<T> {
    head: Option<Link<T>>,
    tail: Option<Link<T>>,
    size: usize,
}

pub struct DoubleLinkedListIter<T> {
    next: Option<Link<T>>,
    next_back: Option<Link<T>>,
}

impl<T> DoubleLinkedList<T> {
    pub fn new() -> Self {
        DoubleLinkedList {
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn insert_at_head(&mut self, data: T) {
        let new_node = Arc::new(Mutex::new(Node::new(data)));
        match self.head.take() {
            None => {
                self.head = Some(Arc::clone(&new_node));
                self.tail = Some(new_node);
                self.size = 1;
            }
            Some(h) => {
                h.lock().unwrap().previous = Some(Arc::clone(&new_node));
                new_node.lock().unwrap().next = Some(h);
                self.head = Some(new_node);
                self.size += 1;
            }
        }
    }

    pub fn insert_at_tail(&mut self, data: T) {
        let new_node = Arc::new(Mutex::new(Node::new(data)));
        match self.tail.take() {
            None => {
                self.head = Some(Arc::clone(&new_node));
                self.tail = Some(new_node);
                self.size = 1;
            }
            Some(t) => {
                t.lock().unwrap().next = Some(Arc::clone(&new_node));
                new_node.lock().unwrap().previous = Some(t);
                self.tail = Some(new_node);
                self.size += 1;
            }
        }
    }

    pub fn pop_head(&mut self) -> Option<T> {
        self.head.take().map(|h| {
            self.size -= 1;
            match h.lock().unwrap().next.take() {
                None => {
                    self.tail.take();
                }
                Some(h_next) => {
                    h_next.lock().unwrap().previous = None;
                    self.head = Some(h_next);
                }
            }
            Arc::try_unwrap(h).ok().unwrap().into_inner().unwrap().item
        })
    }

    pub fn pop_tail(&mut self) -> Option<T> {
        self.tail.take().map(|t| {
            self.size -= 1;
            match t.lock().unwrap().previous.take() {
                None => {
                    self.head.take();
                }
                Some(h_previous) => {
                    h_previous.lock().unwrap().next = None;
                    self.tail = Some(h_previous);
                }
            }
            Arc::try_unwrap(t).ok().unwrap().into_inner().unwrap().item
        })
    }

    pub fn iter<'a>(&'a self) -> DoubleLinkedListIter<T> {
        DoubleLinkedListIter {
            next: self.head.clone(),
            next_back: self.tail.clone(),
        }
    }
}
 
impl<T> Drop for DoubleLinkedList<T> {
    fn drop(&mut self) {
        while let Some(node) = self.head.take() {
            let _ = node.lock().unwrap().previous.take();
            self.head = node.lock().unwrap().next.take();
        }
        self.tail.take();
    }
}

impl<T> Iterator for DoubleLinkedList<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.pop_head()
    }
}

impl<T> DoubleEndedIterator for DoubleLinkedList<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.pop_tail()
    }
}

impl<T> Iterator for DoubleLinkedListIter<T>
where
    T: Clone + Default,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            let guard = node.lock().unwrap();
            self.next = guard.next.clone();
            guard.item.clone()
        })
    }
}

impl<T> DoubleEndedIterator for DoubleLinkedListIter<T>
where
    T: Clone + Default,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.next_back.take().map(|node| {
            let guard = node.lock().unwrap();
            self.next_back = guard.previous.clone();
            guard.item.clone()
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut list = DoubleLinkedList::<i32>::new();
        for i in 0..4 {
            list.insert_at_tail(i);
        }
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), Some(3));
        assert_eq!(iter.next_back(), Some(2));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next_back(), Some(0));
        assert_eq!(iter.next_back(), None);
    }
    
}
