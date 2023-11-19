use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex, RwLock};

use serde::{Deserialize, Serialize};

use crate::ticket::TicketList;

#[derive(thiserror::Error, Debug)]
#[error("Unknown class code {0}")]
pub struct UnknownClass(ClassCode);

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassCode(u16);

impl ClassCode {
    pub fn as_u16(&self) -> u16 {
        self.0
    }
}

impl fmt::Display for ClassCode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:04X}", self.0)
    }
}

#[derive(Clone)]
pub struct AppState {
    classes: Arc<RwLock<HashMap<ClassCode, TicketList>>>,
}

impl AppState {
    pub fn init() -> AppState {
        AppState {
            classes: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn get_unique_code(&self) -> ClassCode {
        let classes = self.classes.read().unwrap();

        loop {
            let code = ClassCode(rand::random());

            if !classes.contains_key(&code) {
                break code;
            }
        }
    }

    pub fn create_class(&self) -> ClassCode {
        let code = self.get_unique_code();
        let mut classes = self.classes.write().unwrap();

        classes.insert(code, TicketList::new(code));

        code
    }

    pub fn get_code(&self, n: u16) -> Result<ClassCode, UnknownClass> {
        let classes = self.classes.read().unwrap();
        let code = ClassCode(n);

        if classes.contains_key(&code) {
            Ok(code)
        } else {
            Err(UnknownClass(code))
        }
    }

    pub fn with_tickets<T>(&self, code: ClassCode, op: impl Fn(&TicketList) -> T) -> T {
        let classes = self.classes.read().unwrap();
        let tickets = classes.get(&code).unwrap();

        op(tickets)
    }

    pub fn with_tickets_mut<T>(&self, code: ClassCode, op: impl Fn(&mut TicketList) -> T) -> T {
        let mut classes = self.classes.write().unwrap();
        let tickets = classes.get_mut(&code).unwrap();

        op(tickets)
    }
}
