
use std::cell::{RefCell, Ref, RefMut};
use std::rc::{Rc};
use std::fmt;
use std::mem;

pub enum Colour { 
    White, 
    Black
}

pub trait Collectable {
    fn mark(&mut self, col: Colour);
    fn get_colour(self);
}


/* The starting-size of a Pile, unless explicitly otherwise */
const DEFAULT_CAPACITY: usize = 8;

#[derive(Debug)]
pub struct Pile {
    memory:         Vec<Entry<u8>>,
    handles:        Vec<Handle>,
    free_head:      Option<usize>,
    handle_head:    Option<usize>,
    allocated:      usize,
}

type PileReference<T> = Rc<RefCell<T>>;

#[derive(Debug)]
enum Entry<T> {
    Free    { next:     Option<usize> },
    Value   { value:    Rc<RefCell<T>> },
}

#[derive(Debug)]
enum Handle {
    Unused  { next: Option<usize> },
    Used    { addr: usize },
}

#[derive(Debug, Copy, Clone)]
pub struct Pointer {
    handle:   usize,
}

impl Pile {
    pub fn new() -> Pile {
        // Create a new pile with the default size
        Pile::with_capacity(DEFAULT_CAPACITY)
    }

    pub fn with_capacity(n: usize) -> Pile {
        // Creates a new pile with a specific capacity n.
        let mut pile = Pile {
            memory:         Vec::new(),
            handles:        Vec::new(),
            free_head:      None,
            handle_head:    None,
            allocated:      0,
        };
        pile.reserve(n);
        pile.reserve_handles(n);
        pile
    }

    pub fn alloc(&mut self, t: u8) -> Pointer {
        match self.try_alloc(t) {
            Ok(ptr) => ptr,
            Err(u)  => self.grow_and_alloc(u), // u is t, back from try_alloc
        }
    }

    pub fn free(&mut self, p: Pointer) {
        let address: usize = match self.handles[p.handle] {
            Handle::Used { addr}  => addr,
            Handle::Unused { .. } => return,
        };

        // replace the element with a free block
        mem::replace(
            &mut self.memory[address],
            Entry::Free {
                next: self.free_head,
            },
        );

        self.free_head = Some(address);
        self.allocated -= 1;
    }

    pub fn get(&self, p: Pointer) -> Option<PileReference<u8>> {
        let address: usize = match self.handles[p.handle] {
            Handle::Used { addr}  => addr,
            Handle::Unused { .. } => return None,
        };
        
        match self.memory.get(address) {
            Some(Entry::Value { value }) => Some(value.clone()),
            _                            => None
        }

    }

    pub fn reserve(&mut self, n: usize) {
        // Adds n indeces to the memory.
        // If we were clever, we would not do anything if the
        // memory can already compensate n elements.
        let old_size = self.memory.len();
        let new_size = old_size + n;
        let old_head = self.free_head;
        self.memory.reserve(n);
        self.memory.extend((old_size..new_size).map(|i| {
            if i == new_size - 1 {
                // The last element in the extended memory is
                // pointing to the previous list of free memory.
                Entry::Free { next: old_head }
            } else {
                // Every other element points to the next. Since
                // we just extended the memory, it is obviously free.
                Entry::Free { next: Some(i + 1) }
            }
        }));
        self.free_head = Some(old_size);
    }

    fn try_alloc(&mut self, t: u8) -> Result<Pointer, u8> {
        // note about return type:
        // we move t, so if we can't insert it we need to give it back :)
        match self.free_head {
            None => Err(t),
            Some(i) => match self.memory[i] {
                Entry::Value { .. } => panic!("corrupt free list"),
                Entry::Free { next } => {
                    self.free_head = next;
                    self.allocated += 1;
                    self.memory[i] = Entry::Value {
                        value: Rc::new(RefCell::new(t))
                    };

                    Ok(Pointer {
                        handle: self.get_handle(i),
                    })
                }
            },
        }
    }

    fn grow_and_alloc(&mut self, t: u8) -> Pointer {
        let len = self.memory.len();
        self.reserve(len); // double length each time, possibly tweak this
        self.try_alloc(t)
            .map_err(|_| ())
            .expect("inserting will always succeed after reserving additional space")
    }

    fn get_handle(&mut self, address: usize) -> usize {
        match self.try_get_handle(address) {
            Ok(h) => h,
            Err(u)  => self.grow_and_get_handle(u),
        }
    }

    fn try_get_handle(&mut self, address: usize) -> Result<usize, usize> {
        // note about return type:
        // we move t, so if we can't insert it we need to give it back :)
        match self.handle_head {
            None => Err(address),
            Some(i) => match self.handles[i] {
                Handle::Used { .. } => panic!("corrupt handle list"),
                Handle::Unused { next } => {
                    self.handle_head = next;
                    self.handles[i] = Handle::Used {
                        addr: address
                    };

                    Ok(i)
                }
            },
        }
    }

    fn grow_and_get_handle(&mut self, address: usize) -> usize {
        let len = self.handles.len();
        self.reserve_handles(len); // double length each time, possibly tweak this
        self.try_get_handle(address)
            .map_err(|_| ())
            .expect("inserting will always succeed after reserving additional space")
    }

    fn reserve_handles(&mut self, n: usize) {
        let old_size = self.handles.len();
        let new_size = old_size + n;
        let old_head = self.handle_head;
        self.handles.reserve(n);
        self.handles.extend((old_size..new_size).map(|i| {
            if i == new_size - 1 {
                // The last element in the extended memory is
                // pointing to the previous list of free memory.
                Handle::Unused { next: old_head }
            } else {
                // Every other element points to the next. Since
                // we just extended the memory, it is obviously free.
                Handle::Unused { next: Some(i + 1) }
            }
        }));
        self.handle_head = Some(old_size);
    }
}
