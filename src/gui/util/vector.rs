use druid::Data;
use druid::im::Vector;
use druid::widget::ListIter;

#[derive(Clone)]
pub struct VectorWrapper<T: Data>(pub(crate) Vector<T>);

impl<T: Data> Data for VectorWrapper<T> {
    fn same(&self, other: &Self) -> bool {
        self.0.same(&other.0)
    }
}

impl<T: Data> ListIter<ListEntry<T>> for VectorWrapper<T> {
    fn for_each(&self, mut cb: impl FnMut(&ListEntry<T>, usize)) {
        for (i, item) in self.0.iter().enumerate() {
            let d = ListEntry {
                list: self.0.to_owned(),
                cached_item: item.to_owned(),
                index: i
            };
            cb(&d, i);
        }
    }

    fn for_each_mut(&mut self, mut cb: impl FnMut(&mut ListEntry<T>, usize)) {
        for (i, item) in self.0.clone().iter().enumerate() {
            let mut d = ListEntry {
                list: self.0.clone(),
                cached_item: item.to_owned(),
                index: i
            };
            cb(&mut d, i);
            if !self.0.same(&d.list){
                println!("updating right");
                self.0 = d.list;
            }
        }
    }

    fn data_len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Clone, Data)]
pub struct ListEntry<T: Data>{
    pub list: Vector<T>,
    pub cached_item: T,
    pub index: usize
}

impl<T: Data> ListEntry<T> {

    pub fn value(&self) -> &T {
        &self.cached_item
    }

    pub fn is_first(&self) -> bool {
        self.index == 0
    }

    pub fn is_last(&self) -> bool  {
        self.index == self.list.len() - 1
    }

    pub fn value_mut(&mut self) -> &mut T {
        println!("invalidate cache");
        &mut self.list[self.index]
    }

    pub fn delete(&mut self) {
        println!("invalidate item");
        self.list.remove(self.index);
    }

    pub fn swap(&mut self, new_index: usize){
        println!("invalidate cache");
        self.list.swap(self.index, new_index);
    }

    pub fn move_relative(&mut self, offset: i32) {
        if offset.is_negative(){
            self.swap(self.index.saturating_sub(offset.abs() as usize));
        } else {
            self.swap(self.index.saturating_add(offset.abs() as usize));
        }

    }

}