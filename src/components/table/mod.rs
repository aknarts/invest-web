pub mod error;
mod macros;
pub mod types;

use std::cmp::Reverse;
use tracing::debug;
use types::{Column, Table, TableData, TableOrder, TableState};
use yew::html;
use yew::prelude::*;

#[derive(Clone, Eq, PartialEq, Default)]
pub struct TableOptions {
    unordered_class: Option<String>,
    ascending_class: Option<String>,
    descending_class: Option<String>,
    orderable_classes: Vec<String>
}

/// Properties of the Table component.
#[derive(Properties, Clone, Eq, PartialEq, Default)]
pub struct Props<T>
where
    T: TableData,
{
    pub columns: Vec<Column>,
    pub data: Vec<T>,
    #[prop_or(false)]
    pub orderable: bool,
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_default]
    pub search: Option<String>,
    #[prop_or_default]
    pub options: TableOptions,
}

#[derive(Debug)]
pub enum Msg {
    SortColumn(usize),
}

impl<T> Component for Table<T>
where
    T: TableData,
{
    type Message = Msg;
    type Properties = Props<T>;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        let column_number = props.columns.len();
        Self {
            columns: props.columns.clone(),
            data: props.data.clone(),
            orderable: props.orderable,
            state: TableState {
                order: vec![TableOrder::default(); column_number],
            },
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SortColumn(i) => {
                use TableOrder::{Ascending, Descending, Unordered};

                for (j, x) in self.state.order.iter_mut().enumerate() {
                    if j == i {
                        *x = x.rotate();
                    } else {
                        *x = Unordered;
                    }
                }

                match self.columns.get(i) {
                    None => false,
                    Some(column) => match column.data_property.as_ref() {
                        Some(f) => match self.state.order.get(i) {
                            Some(order) => {
                                match order {
                                    Unordered => self.data.sort(),
                                    Ascending => self
                                        .data
                                        .sort_by_cached_key(|x| x.get_field_as_value(f).unwrap()),
                                    Descending => self.data.sort_by_cached_key(|x| {
                                        Reverse(x.get_field_as_value(f).unwrap())
                                    }),
                                }
                                true
                            }
                            None => false,
                        },
                        None => false,
                    },
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let search = ctx.props().search.clone();
        let classes = ctx.props().classes.clone();
        html! (
            <table class={classes!(classes)}>
                <thead>
                    { for self.columns.iter().enumerate().map(|(i, col)| self.view_column(ctx, i, col)) }
                </thead>
                <tbody>
                    { for self.data.iter().map(|d| self.view_row(d, search.clone())) }
                </tbody>
            </table>
        )
    }
}

impl<T> Table<T>
where
    T: TableData,
{
    fn view_column<'a>(&'a self, ctx: &Context<Self>, index: usize, column: &'a Column) -> Html {
        let get_header_sorting_class = |index: usize| {
            use TableOrder::{Ascending, Descending, Unordered};

            self.state.order.get(index).map_or(None, |order| match order {
                Unordered => ctx.props().options.unordered_class.clone(),
                Ascending => ctx.props().options.ascending_class.clone(),
                Descending => ctx.props().options.descending_class.clone(),
            })
        };
        let th_view = |child| {
            if self.orderable && column.orderable {
                html! ( <th scope="col" onclick={ctx.link().callback(move |_| Msg::SortColumn(index))}>{ child }</th> )
            } else {
                html! ( <th scope="col">{ child }</th> )
            }
        };

        th_view(html!(
                <span>
                    { column }
                    if self.orderable && column.orderable {
                        <i class={classes!(ctx.props().options.orderable_classes.clone(), get_header_sorting_class(index))}></i>
                    }
                </span>
        ))
    }

    fn view_row(&self, row: &T, search: Option<String>) -> Html {
        if row.matches_search(search) {
            html!(
                <tr>
                    {
                        for self.columns.iter()
                            .map(|c| { c.data_property.as_ref().unwrap_or(&c.name) })
                            .map(|name| { row.get_field_as_html(name) })
                            .filter_map(std::result::Result::ok)
                            .map(|el| html! { <td>{ el }</td> })
                    }
                </tr>
            )
        } else {
            html!()
        }
    }
}
