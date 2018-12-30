use std::mem;
use std::fmt;

#[derive(Debug)]
pub struct FreeList<T> {
    memory: Vec<Entry<T>>,
    head:   Option<usize>,
    len:    usize,
}

#[derive(Debug)]
enum Entry<T> {
    Free  { next: Option<usize> },
    Taken { value: T },
}

const DEFAULT_CAPACITY: usize = 16;

impl<T> FreeList<T> {
    pub fn new() -> FreeList<T> {
        FreeList::with_capacity(DEFAULT_CAPACITY)
    }

    pub fn with_capacity(n: usize) -> FreeList<T> {
        let mut fl = FreeList {
            memory: Vec::new(),
            head:   None,
            len:    0,
        };
        fl.grow(n);
        fl
    }

    pub fn insert(&mut self, t: T) -> usize {
        match self.try_insert(t) {
            Ok(i)  => i,
            Err(t) => self.grow_and_insert(t)
        }
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        match self.memory.get(i) {
            Some(Entry::Taken { ref value }) => Some(value),
            _                                => None
        }
    }
    
    pub fn remove(&mut self, i: usize) -> Option<T> {
        /* We need ownership of the old entry, hence mem::replace */
        let entry = mem::replace(
            &mut self.memory[i],
            Entry::Free { next: self.head }
        );

        match entry {
            Entry::Taken { value: t } => {
                self.head = Some(i);
                self.len -= 1;
                Some(t)
            },
            e @ Entry::Free { .. } => {
                /* If was already free; put it back to preserve list */
                self.memory[i] = e;
                None
            },
        }
    }

    pub fn grow(&mut self, n: usize) {
        let old_len = self.memory.len();
        let new_len = old_len + n;
        let old_head = self.head;
        self.memory.reserve(n);

        for i in old_len .. new_len {
            if i == new_len - 1 {
                self.memory.push(Entry::Free { next: old_head });
            } else {
                self.memory.push(Entry::Free { next: Some(i + 1) });
            }
        }

        self.head = Some(old_len);
    }

    fn try_insert(&mut self, t: T) -> Result<usize, T> {
        if let Some(i) = self.head {
            if let Entry::Free { next } = self.memory[i] {
                self.head = next;
                self.len += 1;
                self.memory[i] = Entry::Taken { value: t };
                Ok(i)
            } else {
                panic!("corrupt free list");
            }
        } else {
            Err(t)
        }
    }

    fn grow_and_insert(&mut self, t: T) -> usize {
        /* Double the length */
        let len = self.memory.len();
        self.grow(len);

        /* Allocate t */
        self.try_insert(t)
            .map_err(|_| "can not fail after growing")
            .unwrap()
    }
}

impl<T: fmt::Debug> fmt::Display for FreeList<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "FreeList. Next insert: {:?}. Len: {}.", self.head, self.len)?;
        
        writeln!(f, "Memory:")?;
        for (i, v) in self.memory.iter().enumerate() {
            writeln!(f, "({}) \t{:?}", i, v)?;
        }

        Ok(())
    }
}
