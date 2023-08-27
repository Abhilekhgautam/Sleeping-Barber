// there are n waiting customer in a barber shop queue and only one barber
// when a new customer arrives, the customer checks if he could acquire a barber
// if not goes to the queue and wait.

// Printing to console is useless, 2 possible reasons:
// 1. My logic is wrong somewhere, so prints out of order
// 2. this is concurrent programming and stdout is locked, so thread gets blocked trying to write
//     and you know what could happen :) (Most Probable Reason)

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
struct Customer(u32);

enum AcquireStatus {
    Success,
    Failed,
}

struct Barber;
struct Shop {
    barber: Mutex<Barber>,
    queue: Mutex<VecDeque<Customer>>,
}

impl Shop {
    fn new() -> Self {
        Self {
            barber: Mutex::new(Barber),
            queue: Mutex::new(VecDeque::new()),
        }
    }

    fn enter_shop(&self, customer: Customer) {
        //let customer_id = customer.0;
        let status = self.acquire_barber(&customer);
        match status {
            AcquireStatus::Success => self.get_hair_cut(&customer),
            AcquireStatus::Failed => {
                self.queue.lock().unwrap().push_back(customer);
                // println!("Customer {} is waiting in the queue", customer_id);
            }
        }
    }

    fn acquire_barber(&self, _customer: &Customer) -> AcquireStatus {
        //let customer_id = customer.0;
        // check queue: if queue is empty try acquiring the barber else get to queue
        let queue_len = self.queue.lock().unwrap().len();
        if queue_len == 0 {
            let barber = &self.barber.try_lock();
            if let Ok(_) = barber {
                // println!("Customer {} will have a hair cut", customer_id);
                return AcquireStatus::Success;
            } else {
                return AcquireStatus::Failed;
            }
        } else {
            return AcquireStatus::Failed;
        }
    }

    fn get_hair_cut(&self, customer: &Customer) {
        thread::sleep(Duration::from_millis(500));
        // println!("Customer {} got a hair cut", customer.0);
        self.leave_shop(customer);
    }

    fn leave_shop(&self, customer: &Customer) {
        // println!("Customer {} left the shop", customer.0);

        // call acquire_barber for the first customer in queue
        let next_customer = self.queue.lock().unwrap().pop_front();
        match next_customer {
            Some(c) => {
                let status = self.acquire_barber(&c);
                match status {
                    AcquireStatus::Success => self.get_hair_cut(customer),
                    _ => unreachable!("Should never reach here, Evil laughter.."),
                }
            }
            None => { // barber goes to sleep
            }
        }
    }
}

fn main() {
    let my_shop = Arc::new(Shop::new());
    let mut thread_vec = Vec::new();
    for i in 1..=500 {
        let my_shop = Arc::clone(&my_shop);
        let j = thread::spawn(move || {
            //a thread represents a customer
            let my_customer = Customer(i);
            my_shop.enter_shop(my_customer);
            thread::sleep(Duration::from_secs(1));
        });
        thread_vec.push(j);
    }
    for handles in thread_vec {
        handles.join().unwrap();
    }
}
