use core::hash::Hash;
use core::ops::{Deref, DerefMut};
use hashbrown::HashMap;
use spin::mutex::SpinMutex;
use spin::once::Once;

// Owned, maybe?
pub struct PopGuard<'src, K, V>
where
    V: Hash,
    K: Eq + Hash,
{
    value: Option<V>,
    key: Option<K>,
    source: Source<'src, K, V>,
}

enum Source<'a, K, V> {
    Return(&'a SpinMutex<Once<HashMap<K, V>>>),
    Forget,
}

impl<'src, K: Eq + Hash, V: Hash> Drop for PopGuard<'src, K, V> {
    fn drop(&mut self) {
        match self.source {
            Source::Forget => {}
            Source::Return(x) => match (self.key.take(), self.value.take()) {
                // uhhh
                (Some(key), Some(value)) => unsafe {
                    let mut lock = x.lock();
                    let lock = lock.get_mut_unchecked();
                    lock.insert(key, value);
                },
                _ => {}
            },
        }
    }
}

impl<'src, K: Eq + Hash, V: Hash> Deref for PopGuard<'src, K, V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        self.value.as_ref().unwrap()
    }
}

impl<'src, K: Eq + Hash, V: Hash> DerefMut for PopGuard<'src, K, V>
where
    V: Hash,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value.as_mut().unwrap()
    }
}

impl<'src, K: Eq + Hash, V: Hash> PopGuard<'src, K, V> {
    pub fn new(key: K, value: V, source: &'src SpinMutex<Once<HashMap<K, V>>>) -> Self {
        Self {
            value: Some(value),
            key: Some(key),
            source: Source::Return(source),
        }
    }

    pub fn no_src(value: V) -> Self {
        Self {
            value: Some(value),
            key: None,
            source: Source::Forget,
        }
    }

    pub fn take(mut self) -> V {
        self.value.take().unwrap()
    }
}
