use crate::repository::Repository;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Book {
    pub id: u32,
    pub name: String,
    pub author: String,
}

#[derive(Clone)]
pub struct BookMemoryRepository {
    repository: Arc<Mutex<HashMap<u32, Book>>>,
}
unsafe impl Send for BookMemoryRepository {}
unsafe impl Sync for BookMemoryRepository {}
impl BookMemoryRepository {
    pub fn new() -> Self {
        BookMemoryRepository {
            repository: Arc::new(Mutex::from(HashMap::new())),
        }
    }
    fn inner(&self) -> MutexGuard<HashMap<u32, Book>> {
        self.repository.lock().unwrap()
    }

    fn new_key(&self) -> u32 {
        let mut key = 1;
        while self.inner().contains_key(&key) {
            key += 1;
        }
        key
    }
}
impl Repository<Book, u32> for BookMemoryRepository {
    fn save(&mut self, mut book: Book) -> u32 {
        let mut key = book.id;
        if book.id == 0 || self.inner().contains_key(&book.id) {
            key = self.new_key();
            book.id = key;
        }
        self.inner().insert(key, book);
        key
    }

    fn get_by_id(&self, id: &u32) -> Option<Book> {
        self.inner().get(id).cloned()
    }

    fn update(&mut self, book: Book) -> Option<Book> {
        self.inner().insert(book.id, book)
    }

    fn delete(&mut self, id: &u32) -> Option<Book> {
        self.inner().remove(id)
    }

    fn get_all(&self) -> Vec<Book> {
        self.inner().values().cloned().collect()
    }
}
