use std::mem;
use std::fmt;

#[derive(Debug)]
pub struct HandleMap2<T> {
    data:           Vec<Entry<T>>,
    slots:          Vec<Slot>,
    free_data_head: Option<usize>,
    free_slot_head: Option<usize>
}

#[derive(Debug)]
enum Entry<T> {
    Free  { next: Option<usize> },
    Taken { value: T },
}

#[derive(Debug)]
struct Slot {
    generation: usize, /* used to invalidate refrences */
    address:    Entry<usize>, /* index in the data vector */
}

#[derive(Debug, Clone, Copy)]
pub struct Handle {
    generation: usize, 
    slot:       usize, /* index in the slots vector */
}

impl<T> HandleMap2<T> {
    pub fn new() -> HandleMap2<T> {
        HandleMap2 {
            data:           Vec::new(),
            slots:          Vec::new(),
            free_data_head: None,
            free_slot_head: None,
        }
    }

    pub fn insert(&mut self, t: T) -> Handle {
        /* Get an address, create a slot, return a handle */
        if let Some(addr) = self.free_data_head {
            /* data[addr] is reusable memory */
            match self.data[addr] {
                Entry::Taken { .. }   => panic!("corrupt free (data) list"),
                Entry::Free  { next } => {
                    self.free_data_head = next;
                    self.data[addr] = Entry::Taken { value: t };
                    self.get_handle(addr)
                }
            }
        } else {
            /* No reusable memory */
            self.data.push(Entry::Taken { value: t });
            let addr = self.data.len() - 1;
            self.get_handle(addr)
        }
    }

    pub fn remove(&mut self, h: Handle) -> Option<T> {
        if !self.is_handle_valid(h) {
            return None;
        }

        let addr = match self.slots[h.slot].address {
            /* Slot is already free */
            Entry::Free  { .. }    => return None,
            /* Slot is taken by an address */
            Entry::Taken { value } => value
        };

        /* recycle the slot */
        self.slots[h.slot].address = Entry::Free {
            next: self.free_slot_head
        };
        self.free_slot_head = Some(h.slot);

        /* recylcle the memory address */
        let old = mem::replace(
            &mut self.data[addr],
            Entry::Free {
                next: self.free_data_head,
            },
        );
        self.free_data_head = Some(addr);

        /* return the old value */
        match old {
            Entry::Free  { .. }    => None,
            Entry::Taken { value } => Some(value)
        }
    }

    fn get_handle(&mut self, addr: usize) -> Handle {
        /* Create/Reuse a slot, and get a handle to it */
        if let Some(n) = self.free_slot_head {
            /* Slot #n is reusable */
            let gen = self.slots[n].generation;
            
            self.free_slot_head = match self.slots[n].address {
                Entry::Taken { .. }   => panic!("corrupt free (slot) list"),
                Entry::Free  { next } => next,
            };

            self.slots[n] = Slot {
                generation: gen + 1,
                address:    Entry::Taken { value: addr }
            };

            Handle {
                generation: gen + 1,
                slot:       n
            }
        } else {
            /* No reusable slots */
            self.slots.push(Slot {
                generation: 1,
                address:    Entry::Taken { value: addr }
            });

            Handle {
                generation: 1,
                slot:       self.slots.len() - 1
            }
        }

    }

    pub fn get(&self, h: Handle) -> Option<&T> {
        if !self.is_handle_valid(h) {
            return None;
        }

        let addr = match self.slots[h.slot].address {
            Entry::Free  { .. } => return None,
            Entry::Taken { value } => value,
        };

        match self.data[addr] {
            Entry::Free  { .. }    => None,
            Entry::Taken { ref value } => {
                Some(value)
            },
        }
    }

    pub fn get_mut(&mut self, h: Handle) -> Option<&mut T> {
        if !self.is_handle_valid(h) {
            return None;
        }

        let addr = match self.slots[h.slot].address {
            Entry::Free  { .. } => return None,
            Entry::Taken { value } => value,
        };

        match self.data[addr] {
            Entry::Free  { .. }    => None,
            Entry::Taken { ref mut value } => {
                Some(value)
            },
        }
    }

    fn is_handle_valid(&self, h: Handle) -> bool {
        self.slots[h.slot].generation == h.generation
    }
}


impl<T: fmt::Debug> fmt::Display for HandleMap2<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "HandleMap2. Next Memory: {:?}, Next Slot: {:?}",
            self.free_data_head, self.free_slot_head)?;
        
        writeln!(f, "Data:")?;
        for (i, v) in self.data.iter().enumerate() {
            writeln!(f, "({}) \t{:?}", i, v)?;
        }

        writeln!(f, "Slots:")?;
        for (i, v) in self.slots.iter().enumerate() {
            writeln!(f, "({}) \t{:?}", i, v)?;
        }


        Ok(())
    }
}
