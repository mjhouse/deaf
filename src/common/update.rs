use std::any::{Any,TypeId};
use std::sync::Mutex;
use once_cell::sync::Lazy;
use std::marker::PhantomData;

/// Placeholder target for Update<T>
pub struct All {}

/// The global instance of the queue of updates
static QUEUE: Lazy<Mutex<UpdateQueue>> = 
    Lazy::new(|| 
        Mutex::new(
            UpdateQueue::new()));

/// The queue of updates to be applied
struct UpdateQueue {
    queue: Vec<Box<dyn Any + Send + Sync>>
}

impl UpdateQueue {

    /// Create an empty queue
    fn new() -> Self {
        Self { queue: Vec::new() }
    }

    /// Clear all updates regardless of type
    pub(crate) fn clear_all(&mut self) {
        self.queue
            .clear();
    }

    /// Clear updates that target type `T`
    pub(crate) fn clear<T: Send + Sync + 'static>(&mut self) {
        self.queue
            .retain(|i| !i
                .downcast_ref::<Update<T>>()
                .is_some())
    }

    /// Add an update that targets type `T`
    pub(crate) fn add<T: Send + Sync + 'static>(&mut self, item: Box<Update<T>>) {
        self.queue
            .push(item);
    }

    /// Find updates that target type `T`
    pub(crate) fn find<T: Send + Sync + 'static>(&self) -> Vec<&Update<T>> {
        self.queue
            .iter()
            .filter_map(|i| i.downcast_ref::<Update<T>>())
            .collect()
    }

    /// Count all updates in the queue
    pub(crate) fn count_all(&self) -> usize {
        self.queue
            .iter()
            .count()
    }

    /// Count updates that target type `T`
    pub(crate) fn count<T: Send + Sync + 'static>(&self) -> usize {
        self.queue
            .iter()
            .filter_map(|i| i.downcast_ref::<Update<T>>())
            .count()
    }

}

/// Update for an object of type `T`
pub struct Update<T: Send + Sync> {
    target: PhantomData<T>,
    change: Box<dyn Fn(&mut T) + Send + Sync>
}

impl<T: Send + Sync +'static> Update<T> {
    
    /// Create a new Update for type `T`
    fn new(change: impl Fn(&mut T) + Send + Sync + 'static) -> Self {
        Self {
            target: PhantomData {},
            change: Box::new(change),
        }
    }

    /// Convenience method to create a Box<Update>
    fn with(change: impl Fn(&mut T) + Send + Sync + 'static) -> Box<Self> {
        Box::new(Self::new(change))
    }

    /// Check if type `T` is `All`
    fn all() -> bool {
        TypeId::of::<All>() == TypeId::of::<T>()
    }

    /// Apply all updates for type `T`
    pub fn apply(target: &mut T) {
        for item in QUEUE.lock().unwrap().find::<T>() {
            (item.change)(target);
        }
    }

    /// Add a new update for type `T`
    pub fn add(change: impl Fn(&mut T) + Send + Sync + 'static) {
        QUEUE.lock().unwrap().add(Update::<T>::with(change));
    }

    /// Clear all updates of type `T`
    pub fn clear() {
        if Update::<T>::all() {
            QUEUE.lock().unwrap().clear_all();
        } else {
            QUEUE.lock().unwrap().clear::<T>();
        }
    }

    /// Count updates of type `T`
    pub fn count() -> usize {
        if Update::<T>::all() {
            QUEUE.lock().unwrap().count_all()
        } else {
            QUEUE.lock().unwrap().count::<T>()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Section;
    use crate::headers::SectionHeader;

    use serial_test::serial;

    struct TestStruct {
        value: u32
    }

    #[test]
    #[serial]
    fn test_create_messages() {
        Update::<All>::clear();
        // SETUP
        // ============================================

        let limit = 30;
        let value = 10;

        // create a section header update
        Update::<SectionHeader>::add(move |t| {
            let n = t.name();
            if n > limit {
                t.set_name(n - value);
            }
        });

        // create a test struct update
        Update::<TestStruct>::add(move |t| {
            t.value += 1;
        });

        // ============================================
        // TEARDOWN
        Update::<All>::clear();
    }

    #[test]
    #[serial]
    fn test_clear_messages() {
        Update::<All>::clear();
        // SETUP
        // ============================================

        // clear all updates and check count
        assert_eq!(Update::<All>::count(), 0);

        // add a sectionheader update that does nothing
        Update::<SectionHeader>::add(move |_| {});

        // check count all and count section header are both 1
        assert_eq!(Update::<All>::count(), 1);
        assert_eq!(Update::<SectionHeader>::count(), 1);

        // add a section update that does nothing
        Update::<Section>::add(move |_| {});

        // check all count is 2, section header and 
        // section update counts are both 1
        assert_eq!(Update::<All>::count(), 2);
        assert_eq!(Update::<Section>::count(), 1);
        assert_eq!(Update::<SectionHeader>::count(), 1);

        // clear the section updates
        Update::<Section>::clear();

        // check count all and count section header are both 1
        assert_eq!(Update::<All>::count(), 1);
        assert_eq!(Update::<SectionHeader>::count(), 1);

        // add a section update that does nothing
        Update::<Section>::add(move |_| {});

        // clear all updates
        Update::<All>::clear();

        // check all counts are 0
        assert_eq!(Update::<All>::count(), 0);
        assert_eq!(Update::<Section>::count(), 0);
        assert_eq!(Update::<SectionHeader>::count(), 0);

        // ============================================
        // TEARDOWN
        Update::<All>::clear();
    }

    #[test]
    #[serial]
    fn test_apply_messages() {
        Update::<All>::clear();
        // SETUP
        // ============================================
        
        let increment = 2;

        // create a test struct to update
        let mut test = TestStruct { value: 1 };

        // add an update that applies to type TestStruct
        Update::<TestStruct>::add(move |t| {
            t.value += increment;
        });

        // apply the update to the test struct
        Update::apply(&mut test);

        // verify that value has been incremented
        assert_eq!(test.value, 3);

        // ============================================
        // TEARDOWN
        Update::<All>::clear();
    }

}