//! This module contains the application's internal state. There are no endpoints defined in this module.
//!
//! The [`AppState`] struct wraps over a map of [`ClassCode`]s to their [`TicketList`]s, and provides
//! convenient methods for accessing that state.

use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};

use crate::ticket::TicketList;

/// Error type when an invalid class code is given.
#[derive(thiserror::Error, Debug)]
#[error("Unknown class code {0}")]
pub struct UnknownClass(ClassCode);

/// A class code. Wrapped up in a struct to ensure that any `ClassCode` we have access to is valid
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassCode(u16);

impl ClassCode {
    /// Return the inner `u16` contained in the class code
    pub fn as_u16(&self) -> u16 {
        self.0
    }
}

impl fmt::Display for ClassCode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        // format inner u16 as 4-digit hex number
        write!(fmt, "{:04X}", self.0)
    }
}

/// The application's state. Stores a map of [`ClassCode`] to [`TicketList`]
#[derive(Clone)]
pub struct AppState {
    /// The map of [`ClassCode`] to [`TicketList`], stored in a `RwLock` to support multiple readers, but
    /// only one writer, at any given time.
    classes: Arc<RwLock<HashMap<ClassCode, TicketList>>>,
}

impl AppState {
    /// Create an empty instance of the application state
    pub fn init() -> AppState {
        AppState {
            classes: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Randomly generates class codes, returns the first one that is not yet in use
    fn get_unique_code(&self) -> ClassCode {
        // aquire a read lock on the class list
        let classes = self.classes.read().unwrap();

        loop {
            // generate random code
            let code = ClassCode(rand::random());

            if !classes.contains_key(&code) {
                // code isn't in use, break & return
                break code;
            }
        }
    }

    /// Creates a new class and returns it's unique code
    pub fn create_class(&self) -> ClassCode {
        // get a unique code & acquire write lock on classes
        let code = self.get_unique_code();
        let mut classes = self.classes.write().unwrap();

        // insert empty `TicketList` into class
        classes.insert(code, TicketList::new());

        // return class code
        code
    }

    /// Retrieve a [`ClassCode`] from a `u16`. If the given code is not in use, then produce an error.
    pub fn get_code(&self, id: u16) -> Result<ClassCode, UnknownClass> {
        // acquire read lock on classes & wrap ID in `ClassCode` for comparison
        let classes = self.classes.read().unwrap();
        let code = ClassCode(id);

        if classes.contains_key(&code) {
            // class exists; return code
            Ok(code)
        } else {
            // class doesn't exist; return error
            Err(UnknownClass(code))
        }
    }

    /// Perform an immutable operation on a given class's [`TicketList`]
    pub fn with_tickets<T>(&self, code: ClassCode, op: impl Fn(&TicketList) -> T) -> T {
        // acquire read lock & retrieve reference to ticket list
        let classes = self.classes.read().unwrap();
        let tickets = classes.get(&code).unwrap();

        // perform operation on ticket list
        op(tickets)
    }

    /// Perform mutable operation on a given class's [`TicketList`]
    pub fn with_tickets_mut<T>(&self, code: ClassCode, op: impl Fn(&mut TicketList) -> T) -> T {
        // acquire read lock & retrieve mutable reference to ticket list
        let mut classes = self.classes.write().unwrap();
        let tickets = classes.get_mut(&code).unwrap();

        // perform operation on ticket list
        op(tickets)
    }
}
