#[derive(Debug)]
pub struct HandleMap<T> {
    data:       Vec<T>,
    slots:      Vec<Slot>,
    free_data:  Vec<usize>,
    free_slots: Vec<usize>
}

pub struct HandleMap2<T> {
    data:       Vec<Entry<T>>,
    slots:      Vec<Slot>,
    free_data:  Option<usize>,
    free_slots: Option<usize>
}

pub enum Entry<T> {
    Free  { next: Option<usize> },
    Taken { value: T },
}

#[derive(Debug)]
struct Slot {
    generation: usize, /* used to invalidate refrences */
    address:    usize, /* address in the data vector */
}

#[derive(Debug, Clone, Copy)]
pub struct Handle {
    generation: usize, 
    slot:       usize, /* address in the handles vector */
}

impl<T> HandleMap<T> {
    pub fn new() -> HandleMap<T> {
        HandleMap {
            data:       Vec::new(),
            slots:      Vec::new(),
            free_data:  Vec::new(),
            free_slots: Vec::new(),
        }
    }

    pub fn insert(&mut self, t: T) -> Handle {
        /* Store t and work out the address */
        let addr = match self.free_data.pop() {
            /* Can reuse the adress a */
            Some(a) => {
                self.data[a] = t;
                a
            },
            /* No reusable address */
            None => {
                self.data.push(t);
                self.data.len() - 1
            }
        };

        /* Get a new handle to the adress */
        if let Some(i) = self.free_slots.pop() {
            /* Re-use a slot */
            let current_gen = self.slots[i].generation + 1;
            self.slots[i] = Slot {
                generation: current_gen,
                address:    addr,
            };

            return Handle {
                generation: current_gen,
                slot:       i,
            };
        } else {
            /* No reusable slots */
            self.slots.push(Slot {
                generation: 1,
                address:    addr,
            });

            return Handle { 
                generation: 1,
                slot:       self.slots.len() - 1,
            };
        };
    }

    pub fn remove(&mut self, h: Handle) {
        let generation = self.slots[h.slot].generation;
        
        if h.generation != generation {
            return ();
        }

        let address = self.slots[h.slot].address;

        /* schedule data-address for reuse */
        self.free_data.push(address);

        /* bump up the gen. count, since all handles are now invalid */
        self.slots[h.slot].generation += 1;

        /* schedule handle for reuse */
        self.free_slots.push(h.slot);

    }

    pub fn get(&self, h: Handle) -> Option<&T> {
        let generation = self.slots[h.slot].generation;

        if h.generation != generation {
            return None;
        }

        let address = self.slots[h.slot].address;

        Some(&self.data[address])
    }

    pub fn get_mut(&mut self, h: Handle) -> Option<&mut T> {
        let generation = self.slots[h.slot].generation;

        if h.generation != generation {
            return None;
        }

        let address = self.slots[h.slot].address;

        Some(&mut self.data[address])
    }
}
