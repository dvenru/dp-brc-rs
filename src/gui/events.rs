use super::BarCodeData;

#[derive(Debug)]
pub enum BarAppEvents {
    AddItem(BarCodeData),
    UpdateItem(BarCodeData),
    ItemSelected(BarCodeData),
    ShowHistory(BarCodeData),
    ShowItems,
    CheckNameItem(String),
    ErrorNameItem,
    SwitchTabToUpdate,
}

pub type Events = Vec<BarAppEvents>;

pub trait RemoveMultiple<T> {
    fn remove_multiple(&mut self, remove_vec: Vec<usize>);
}

impl<T> RemoveMultiple<T> for Vec<T> {
    fn remove_multiple(&mut self, mut remove_vec: Vec<usize>) {
        remove_vec.reverse();

        for idx in remove_vec {
            self.remove(idx);
        }
    }
}
