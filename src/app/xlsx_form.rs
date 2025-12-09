use crate::app::cards::CardsServerProps;
use leptos::logging::log;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use std::num::NonZeroUsize;
use std::path::PathBuf;

#[component]
pub fn XlsxForm(title: RwSignal<String>, csp: RwSignal<Option<CardsServerProps>>) -> impl IntoView {
    let title_row_index = RwSignal::new(None);
    let sheetname = RwSignal::<String>::new(String::new());
    let path = RwSignal::<Option<PathBuf>>::new(None);
    let columns_indexs = RwSignal::<Vec<usize>>::new(Vec::new());
    let on_submit = move |_| {
        if let (Some(path), sheet, columns_indexs, false) = (
            path.get(),
            sheetname.get(),
            columns_indexs.get(),
            title.read().is_empty(),
        ) && !sheet.is_empty()
            && !columns_indexs.is_empty()
        {
            let res = Some(CardsServerProps {
                title_row_index: title_row_index.get(),
                path,
                sheet,
                columns_indexs,
            });
            csp.set(res);
        };
    };
    let disabled = move || {
        path.read().is_none()
            || sheetname.read().is_empty()
            || columns_indexs.read().is_empty()
            || title.read().is_empty()
    };
    let submit_title = move || {
        if disabled() {
            "افندم!"
        } else {
            "تمام"
        }
    };
    let submit_style = move || {
        if disabled() {
            "color:red;"
        } else {
            "color:green;"
        }
    };
    view! {
        <dl class="border-sky-500 border-5 rounded-xl p-2 m-2 text-xl text-center">
            <CardTitle title/>
            <XlsxPath path/>
            <SheetName sheetname path/>
            <TitleRowIndex path sheetname=sheetname index=title_row_index/>
            <ColumnsIndexs indexs=columns_indexs sheetname=sheetname path headers_index=title_row_index/>
            <button
                disabled=disabled
                on:click=on_submit
                class="text-3xl font-bold border-2 rounded-xl p-4 hover:cursor-pointer disabled:cursor-wait"
                style=submit_style
            >{submit_title}</button>
        </dl>
    }
}

#[component]
fn CardTitle(title: RwSignal<String>) -> impl IntoView {
    let style = move || {
        if title.read().is_empty() {
            "color:red;"
        } else {
            ""
        }
    };
    view! {
        <dd class="text-2xl m-2 p-2 font-bold border-l-2 border-r-2 rounded-xl">عنوان الكارت</dd>
        <dt>
            <input
                type="text"
                style=style
                class="border-2 w-3/6 rounded-lg p-2 text-center"
                on:input:target=move |ev| {
                    let value =ev.target().value();
                    title.set(value.trim().to_string());
                }
            />
        </dt>
    }
}

#[server]
async fn get_headers(
    args: (Option<PathBuf>, String, Option<NonZeroUsize>),
) -> Result<Vec<String>, ServerFnError> {
    use calamine::{DeError, RangeDeserializerBuilder, Reader, Xlsx, open_workbook};
    let (path, sheetname, headers_index) = args;
    let Some(path) = path else {
        return Ok(Vec::new());
    };
    let mut workbook: Xlsx<_> = open_workbook(&path)?;

    let range = workbook.worksheet_range(&sheetname)?;

    let mut iter = RangeDeserializerBuilder::new()
        .has_headers(false)
        .from_range(&range)?;

    let headers: Vec<String> = match headers_index {
        Some(i) => iter
            .nth(Into::<usize>::into(i) - 1)
            .unwrap_or(Err(DeError::HeaderNotFound(format!(
                "Error number {i} should contain headers"
            ))))?,
        None => iter
            .next()
            .unwrap_or(Err(DeError::HeaderNotFound(String::from(
                "Error : first row should contain headers",
            ))))?,
    };

    Ok(headers)
}

#[component]
fn ColumnsIndexs(
    indexs: RwSignal<Vec<usize>>,
    path: RwSignal<Option<PathBuf>>,
    sheetname: RwSignal<String>,
    headers_index: RwSignal<Option<NonZeroUsize>>,
) -> impl IntoView {
    let headers_res = Resource::new(
        move || (path.get(), sheetname.get(), headers_index.get()),
        get_headers,
    );
    let headers = move || {
        headers_res
            .get()
            .transpose()
            .ok()
            .flatten()
            .unwrap_or_default()
            .into_iter()
            .enumerate()
            .collect::<Vec<_>>()
    };
    let style = move || {
        if indexs.read().is_empty() {
            "color:red;"
        } else {
            ""
        }
    };
    view! {
        <dd class="text-2xl m-2 p-2 font-bold border-l-2 border-r-2 rounded-xl">الاعمدة</dd>
        <dt>
            <dl
                style=style
                class="flex flex-wrap gap-4 place-content-center"
            >
                <Suspense>
                    <For
                        each=headers
                        key=|x| x.1.clone()
                        let((index,header))
                    >
                        <div class="grid grid-cols-1 gap-4 border-2 rounded-xl p-3 m-2">
                            <dd>{header}</dd>
                            <dt>
                                <input
                                    type="checkbox"
                                    class="w-5 h-5"
                                    value={index}
                                    on:change:target=move |ev| {
                                        if ev.target().checked() {
                                            indexs.write().push(ev.target().value().parse().unwrap());
                                        } else {
                                            indexs.write().retain(|x| (*x) != ev.target().value().parse::<usize>().unwrap());
                                        };
                                    }
                                />
                            </dt>
                        </div>
                    </For>
                </Suspense>
            </dl>
        </dt>
    }
}

#[server]
async fn sheets_names(path: Option<PathBuf>) -> Result<Vec<String>, ServerFnError> {
    use calamine::{Reader, Xlsx, open_workbook};
    let Some(path) = path else {
        return Ok(Vec::new());
    };
    let workbook: Xlsx<_> = open_workbook(&path)?;
    Ok(workbook.sheet_names())
}

#[component]
fn SheetName(sheetname: RwSignal<String>, path: RwSignal<Option<PathBuf>>) -> impl IntoView {
    let sheets_names_res = Resource::new(move || path.get(), sheets_names);
    let style = move || {
        if path.read().is_none() {
            "color:red;"
        } else {
            ""
        }
    };

    let sheets_names = move || {
        sheets_names_res
            .get()
            .transpose()
            .ok()
            .flatten()
            .unwrap_or_default()
    };
    view! {
        <dd class="text-2xl m-2 p-2 font-bold border-l-2 border-r-2 rounded-xl">اسم الشييت</dd>
        <dt>
            <select
                style={style}
                class="border-2 w-3/6 rounded-lg p-2 text-center"
                on:change:target=move |ev| {
                    let value =ev.target().value();
                    sheetname.set( value.trim().to_string());
                }
            >
                <option>"لا يكن"</option>
                <Suspense>
                <For
                    each=sheets_names
                    key=|x| x.clone()
                    let(name)
                >
                    <option value={name.clone()}>{name.clone()}</option>
                </For>
                </Suspense>
            </select>
        </dt>
    }
}

#[server]
async fn rows_height(args: (Option<PathBuf>, String)) -> Result<usize, ServerFnError> {
    use calamine::{Reader, Xlsx, open_workbook};
    let (path, sheetname) = args;
    let (Some(path), false) = (path, sheetname.is_empty()) else {
        return Ok(0);
    };
    let mut workbook: Xlsx<_> = open_workbook(&path)?;
    let Ok(range) = workbook.worksheet_range(&sheetname) else {
        println!("sheet {sheetname} range is empty");
        return Ok(0);
    };
    Ok(range.height())
}

#[component]
fn TitleRowIndex(
    index: RwSignal<Option<NonZeroUsize>>,
    path: RwSignal<Option<PathBuf>>,
    sheetname: RwSignal<String>,
) -> impl IntoView {
    let style = move || {
        if index.read().is_some() {
            return "";
        };
        "color:red;"
    };
    let rows_height_res = Resource::new(move || (path.get(), sheetname.get()), rows_height);
    let rows_height = move || rows_height_res.get().transpose().ok().flatten();
    view! {
        <dd class="text-2xl m-2 p-2 font-bold border-l-2 border-r-2 rounded-xl">مسلسل صف العناوين</dd>
        <dt>
            <select
                style={style}
                class="border-2 w-2/6 rounded-lg p-2 text-center"
                on:change:target=move |ev| {
                    let value =ev.target().value().parse::<NonZeroUsize>();
                    match value {
                        Ok(value) => {
                            index.set(Some(value));
                        },
                        Err(err) => {
                            index.set(None);
                            log!("Error : {err:#?}");
                        },
                    };
                }
            >
                <Suspense>
                    <ShowLet some=rows_height let(size)>
                    {
                        (1..=size).flat_map(NonZeroUsize::new).map(|i| {
                            view! {
                                <option value={i} selected=move || index.read().is_some_and(|x| x == i)>{i}</option>
                            }
                        }).collect_view()
                    }
                    </ShowLet>
                </Suspense>
            </select>
        </dt>
    }
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize, Clone)]
enum PathExisting {
    Exists(PathBuf),
    ParentExists(PathBuf),
    None,
}

#[server]
async fn path_exists(path: PathBuf) -> Result<PathExisting, ServerFnError> {
    let res = if path.exists() {
        PathExisting::Exists(path)
    } else if path.parent().is_some_and(|x| x.exists()) {
        PathExisting::ParentExists(path)
    } else {
        PathExisting::None
    };
    Ok(res)
}

#[server]
async fn path_autocomplete(path: PathExisting) -> Result<Vec<PathBuf>, ServerFnError> {
    match path {
        PathExisting::Exists(path) => {
            let mut enteries = tokio::fs::read_dir(&path).await?;
            let mut paths = Vec::new();
            while let Some(entry) = enteries.next_entry().await? {
                paths.push(entry.path());
            }
            Ok(paths)
        }
        PathExisting::ParentExists(path) => {
            let parent = path.parent().unwrap();
            let name = path.file_name().unwrap().to_str().unwrap();
            let mut enteries = tokio::fs::read_dir(&parent).await?;
            let mut paths = Vec::new();
            while let Some(entry) = enteries.next_entry().await? {
                let epath = entry.path();
                if epath
                    .file_name()
                    .and_then(|x| x.to_str())
                    .is_some_and(|x| x.starts_with(&name))
                {
                    paths.push(epath);
                }
            }
            Ok(paths)
        }
        PathExisting::None => Ok(Vec::new()),
    }
}

#[component]
fn XlsxPath(path: RwSignal<Option<PathBuf>>) -> impl IntoView {
    let input_path = RwSignal::new(PathBuf::new());
    let style = RwSignal::new("");

    let input_path_exists_res = Resource::new(move || input_path.get(), path_exists);
    let input_path_exists = move || {
        input_path_exists_res
            .get()
            .transpose()
            .ok()
            .flatten()
            .unwrap_or(PathExisting::None)
    };

    let autocomplete_paths_res = Resource::new(input_path_exists, path_autocomplete);
    let autocomplete_paths = move || {
        autocomplete_paths_res
            .get()
            .transpose()
            .ok()
            .flatten()
            .unwrap_or_default()
    };

    Effect::new(move || {
        let input_path = input_path.get_untracked();

        let is_excel = input_path
            .extension()
            .is_some_and(|x| ["xls", "xlsx", "xlsb", "ods"].contains(&x.to_str().unwrap_or("")));

        if matches!(input_path_exists(), PathExisting::Exists(_)) && is_excel {
            path.set(Some(input_path));
            style.set("");
        } else {
            style.set("color:red;");
        }
    });

    view! {
        <dd class="text-2xl m-2 p-2 font-bold border-l-2 border-r-2 rounded-xl">موقع ملف الاكسل</dd>
        <dt>
            <input
                dir="ltr"
                type="text"
                class="border-2 w-5/6 rounded-lg p-3 text-center"
                list="paths"
                style=style
                on:input:target=move |ev| {
                    let value =ev.target().value().parse::<PathBuf>();
                    let Ok(value) = value;
                    input_path.set(value);
                }
            />
            <datalist id="paths">
                <Suspense>
                <For
                    each=autocomplete_paths
                    key=|x| x.clone()
                    let(path)
                >
                    <option value={path.display().to_string()}/>
                </For>
                </Suspense>
            </datalist>
        </dt>
    }
}
