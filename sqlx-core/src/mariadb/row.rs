use crate::{
    mariadb::{protocol::ResultRow, MariaDb},
    row::Row,
};

#[derive(Debug)]
pub struct MariaDbRow(pub(crate) ResultRow);

impl Row for MariaDbRow {
    type Backend = MariaDb;

    #[inline]
    fn len(&self) -> usize {
        self.0.values.len()
    }

    #[inline]
    fn get_raw(&self, index: usize) -> Option<&[u8]> {
        self.0.values[index]
            .as_ref()
            .map(|value| unsafe { value.as_ref() })
    }
}