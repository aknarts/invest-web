use super::error::*;
use serde::Serialize;
use serde_value::Value;
use std::fmt;
use yew::Html;

pub trait TableData: 'static + Default + Clone + Ord + Serialize {
    /// Returns the Html representation of a field. When None, the field is not rendered.
    fn get_field_as_html(&self, field_name: &str) -> Result<Html>;

    /// Returns a table value given its field name. This value is used as a sorting key for the corresponding column.
    fn get_field_as_value(&self, field_name: &str) -> Result<Value>;

    fn matches_search(&self, needle: Option<String>) -> bool;
}

#[derive(Clone, PartialEq, Default, Debug)]
pub struct Column {
    pub name: String,
    pub short_name: Option<String>,
    pub data_property: Option<String>,
    pub orderable: bool,
}

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.short_name.as_ref().unwrap_or(&self.name))
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum TableOrder {
    Unordered = 0,
    Ascending,
    Descending,
}

impl Default for TableOrder {
    fn default() -> Self {
        TableOrder::Unordered
    }
}

impl TableOrder {
    pub fn rotate(&self) -> Self {
        use TableOrder::*;
        match *self {
            Unordered => Ascending,
            Ascending => Descending,
            Descending => Unordered,
        }
    }
}

#[derive(Clone, PartialEq, Default)]
pub struct TableState {
    pub order: Vec<TableOrder>,
}

/// The a table with columns holding data.
#[derive(Clone, PartialEq, Default)]
pub struct Table<T>
where
    T: TableData,
{
    /// The order of the columns determines the order in which they are displayed.
    pub columns: Vec<Column>,
    pub data: Vec<T>,
    pub state: TableState,
    pub orderable: bool,
}
