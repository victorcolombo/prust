use crate::RefCounter;

use super::list;

pub struct Deque<T> {
    head: list::List<T>,
    tail: list::List<T>,
}

impl<T> Clone for Deque<T> {
    fn clone(&self) -> Self {
        Self {
            head: self.head.clone(),
            tail: self.tail.clone(),
        }
    }
}

impl<T> Deque<T> {
    pub fn empty() -> Self {
        Self {
            head: list::List::empty(),
            tail: list::List::empty(),
        }
    }
}

impl<T> Deque<T> {
    pub fn push_front(&self, value: T) -> Self {
        Self {
            head: self.head.push_front(value),
            tail: self.tail.clone(),
        }
        .balance()
    }

    pub fn push_back(&self, value: T) -> Self {
        Self {
            head: self.head.clone(),
            tail: self.tail.push_front(value),
        }
        .balance()
    }

    pub fn pop_front(&self) -> Option<(&T, Self)> {
        if self.is_empty() {
            None
        } else if self.head.is_empty() {
            let (a, b) = self.tail.pop_front().unwrap();
            Some((
                a,
                Self {
                    head: self.head.clone(),
                    tail: b,
                },
            ))
        } else {
            let (a, b) = self.head.pop_front().unwrap();
            Some((
                a,
                Self {
                    head: b,
                    tail: self.tail.clone(),
                },
            ))
        }
    }

    pub fn pop_back(&self) -> Option<(&T, Self)> {
        if self.is_empty() {
            None
        } else if self.tail.is_empty() {
            let (a, b) = self.head.pop_front().unwrap();
            Some((
                a,
                Self {
                    head: b,
                    tail: self.tail.clone(),
                },
            ))
        } else {
            let (a, b) = self.tail.pop_front().unwrap();
            Some((
                a,
                Self {
                    head: self.head.clone(),
                    tail: b,
                },
            ))
        }
    }

    fn balance(&self) -> Self {
        if self.head.is_empty() {
            let (tail, rev_head) = self.tail.split();
            let head = rev_head.reverse();
            Self { head, tail }
        } else if self.tail.is_empty() {
            let (head, rev_tail) = self.head.split();
            let tail = rev_tail.reverse();
            Self { head, tail }
        } else {
            self.clone()
        }
    }

    fn is_empty(&self) -> bool {
        self.length() == 0
    }

    fn length(&self) -> usize {
        self.head.length() + self.tail.length()
    }

    pub fn iter(&self) -> DequeIterator<T> {
        DequeIterator {
            head_iter: self.head.iter(),
            tail_iter: self.tail.reverse().iter(),
        }
    }
}

pub struct DequeIterator<T> {
    head_iter: list::ListIterator<T>,
    tail_iter: list::ListIterator<T>,
}

impl<T> Iterator for DequeIterator<T> {
    type Item = RefCounter<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.head_iter.next() {
            Some(value) => Some(value),
            None => self.tail_iter.next(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deque_push_pop() {
        let deque: Deque<i32> = Deque::empty();
        let deque = deque.push_front(1).push_back(2).push_front(0).push_back(3);
        assert_eq!(deque.length(), 4);

        let (value, deque) = deque.pop_front().unwrap();
        assert_eq!(*value, 0);
        let (value, deque) = deque.pop_back().unwrap();
        assert_eq!(*value, 3);
        let (value, deque) = deque.pop_front().unwrap();
        assert_eq!(*value, 1);
        let (value, deque) = deque.pop_back().unwrap();
        assert_eq!(*value, 2);

        assert_eq!(deque.length(), 0);
        assert!(deque.pop_front().is_none());
        assert!(deque.pop_back().is_none());
    }

    #[test]
    fn test_deque_iter() {
        let deque: Deque<String> = Deque::empty();
        let deque = deque
            .push_front("World".to_string())
            .push_front("Hello".to_string());
        let mut iter = deque.iter();
        assert_eq!(iter.next(), Some(RefCounter::new("Hello".to_string())));
        assert_eq!(iter.next(), Some(RefCounter::new("World".to_string())));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn demonstrate_readme() {
        // deque: [2, 1]
        let deque: Deque<i32> = Deque::empty().push_front(1).push_front(2);

        // Even with pop_back, deque is still unaltered, since it's immutable
        let (value, _) = deque.pop_back().unwrap();
        assert_eq!(*value, 1);
        assert_eq!(deque.length(), 2);

        // The "new" deque with pop_back is returned as part of the call
        let (value, deque_updated) = deque.pop_front().unwrap();
        assert_eq!(*value, 2);
        assert_eq!(deque_updated.length(), 1);
    }
}
