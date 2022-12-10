use tracing::debug;
use yew::prelude::*;
use yew::suspense::{use_future, SuspensionResult, UseFutureHandle};
use yew_router::prelude::use_navigator;
use crate::app::Route;
use crate::services::admin::{get_permissions_list, Permission, RoleInfo};
use crate::error::Error;

#[hook]
fn use_permissions_list() -> SuspensionResult<UseFutureHandle<Result<Vec<Permission>, Error>>> {
    use_future(|| async move { get_permissions_list().await })
}

#[derive(Properties, PartialEq)]
pub struct PermissionsProp {
    pub selected_callback: Callback<InputEvent>,
    pub search: Option<String>,
    pub info: RoleInfo,
    pub additional_permissions: Option<Vec<Permission>>,
}

#[function_component(PermissionList)]
pub fn permission_list(props: &PermissionsProp) -> HtmlResult {
    let res = use_permissions_list()?;
    let history = use_navigator().unwrap();
    let html_result = match *res {
        Ok(ref list) => {
            let mut full_list = list.clone();
            if let Some(additional) = &props.additional_permissions {
                for add in additional {
                    if !list.contains(add) {
                        full_list.push(add.clone());
                    }
                }
            }
            full_list.sort_by(|a, b| a.description.cmp(&b.description));

            html!(
                <>
                    {
                        for full_list.iter().map(|permission| {
                            props.search.as_ref().map_or_else(|| html!(
                                    permission_checkbox(&props.selected_callback,
                                                        props.info.permissions.contains(&permission.name),
                                                        permission)
                                ), |s| if permission.description.contains(s) || permission.name.contains(s) {
                                    permission_checkbox(&props.selected_callback,
                                                        props.info.permissions.contains(&permission.name),
                                                        permission)
                                } else {
                                   html!()
                                }
                            )
                        })
                    }
                </>
            )
        }
        Err(ref e) => {
            match e {
                Error::Unauthorized(s) | Error::Forbidden(s) => {
                    debug!("Authorization issue: {}", s);
                }
                _ => {
                    debug!("Failed to complete request: {e}");
                }
            };
            history.push(&Route::Home);
            html!(500)
        }
    };
    Ok(html_result)
}

fn permission_checkbox(
    callback: &Callback<InputEvent>,
    checked: bool,
    permission: &Permission,
) -> Html {
    html!(
        <PermissionLine
           selected_callback={callback}
           checked={ checked }
           name={permission.name.clone()}
           description={permission.description.clone()} />
    )
}

#[derive(Properties, PartialEq)]
pub struct PermissionProp {
    pub selected_callback: Callback<InputEvent>,
    pub checked: bool,
    pub name: String,
    pub description: String,
}

#[function_component(PermissionLine)]
pub fn permission_line(props: &PermissionProp) -> Html {
    html!(
        <div class="form-check form-switch">
            <input
                class="form-check-input"
                type="checkbox"
                id={ props.name.clone() }
                checked={ props.checked }
                value={ props.name.clone() }
                oninput={&props.selected_callback}
                />
            <label class="form-check-label" for={ props.name.clone() }>{ props.description.clone() }</label>
        </div>
    )
}
