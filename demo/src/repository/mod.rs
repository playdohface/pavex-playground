pub mod book;

pub trait Repository<E, I> {
    fn save(&mut self, entity: E) -> I;
    fn get_by_id(&self, id: &I) -> Option<E>;
    fn update(&mut self, entity: E) -> Option<E>;
    fn delete(&mut self, id: &I) -> Option<E>;
    fn get_all(&self) -> Vec<E>;
}
