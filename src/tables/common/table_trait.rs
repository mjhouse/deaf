
pub trait Table<T> {

    fn len(&self) -> usize;

    fn size(&self) -> usize;

    fn get(&self, index: usize) -> Option<T>;

    fn set(&mut self, index: usize, item: T);

    fn add(&mut self, item: T);

    fn del(&mut self, index: usize) -> Option<T>;

}