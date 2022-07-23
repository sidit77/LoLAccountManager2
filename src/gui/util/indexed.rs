use std::ops::{Deref, DerefMut};
use druid::Data;
use druid::im::Vector;
use druid::widget::ListIter;

#[derive(Clone)]
pub struct IndexWrapper<T: Data>(Vector<T>);

impl<T: Data> From<Vector<T>> for IndexWrapper<T> {
    fn from(vec: Vector<T>) -> Self {
        Self(vec)
    }
}

impl<T: Data> From<IndexWrapper<T>> for Vector<T> {
    fn from(wrap: IndexWrapper<T>) -> Self {
        wrap.0
    }
}

impl<T: Data> Data for IndexWrapper<T> {
    fn same(&self, other: &Self) -> bool {
        self.0.same(&other.0)
    }
}

impl<T: Data> ListIter<Indexed<T>> for IndexWrapper<T> {
    fn for_each(&self, mut cb: impl FnMut(&Indexed<T>, usize)) {
        let len = self.0.len();
        for (i, item) in self.0.iter().enumerate() {
            let indexed = Indexed {
                item: item.to_owned(),
                index: i,
                len
            };
            cb(&indexed, i);
        }
    }

    fn for_each_mut(&mut self, mut cb: impl FnMut(&mut Indexed<T>, usize)) {
        let len = self.0.len();
        for (i, item) in self.0.iter_mut().enumerate() {
            let mut indexed = Indexed {
                item: item.to_owned(),
                index: i,
                len
            };
            cb(&mut indexed, i);

            if !item.same(&indexed.item) {
                *item = indexed.item;
            }
        }
    }

    fn data_len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Clone, Data)]
pub struct Indexed<T: Data>{
    pub item: T,
    index: usize,
    len: usize
}

impl<T: Data> Indexed<T> {

    pub fn is_first(&self) -> bool {
        self.index == 0
    }

    pub fn is_last(&self) -> bool {
        self.index == self.len - 1
    }

    pub fn index(&self) -> usize {
        self.index
    }

}

impl<T: Data> Deref for Indexed<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

impl<T: Data> DerefMut for Indexed<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.item
    }
}