use super::BarCodeData;

pub trait EventHandler {
    fn event_handler(&mut self, events: &mut Events);
}

#[derive(Debug)]
pub enum BarAppEvents {
    UpdateTable,
    ItemSelected(BarCodeData),
    ShowItemHistory(BarCodeData),
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


