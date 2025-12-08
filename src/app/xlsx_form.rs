use crate::app::cards::CardsServerProps;
use leptos::logging::log;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use std::num::NonZeroUsize;
use std::path::PathBuf;

#[component]
pub fn XlsxForm(title: RwSignal<String>, csp: RwSignal<Option<CardsServerProps>>) -> impl IntoView {
    let title_row_index = RwSignal::new(NonZeroUsize::new(1));
    let sheet = RwSignal::<String>::new(String::new());
    let path = RwSignal::<Option<PathBuf>>::new(None);
    let columns_indexs = RwSignal::<Vec<usize>>::new(Vec::new());
    let on_submit = move |_| {
        if let (Some(path), sheet, columns_indexs, false) = (
            path.get(),
            sheet.get(),
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
    view! {
        <dl class="border-sky-500 border-5 rounded-xl p-2 m-2 text-xl text-center">
            <CardTitle title/>
            <XlsxPath path/>
            <SheetName sheet/>
            <TitleRowIndex index=title_row_index/>
            <ColumnsIndexs indexs=columns_indexs/>
            <button on:click=on_submit>تمام</button>
        </dl>
    }
}

#[component]
fn CardTitle(title: RwSignal<String>) -> impl IntoView {
    view! {
        <dd class="text-2xl m-2 p-2 font-bold border-l-2 border-r-2 rounded-xl">عنوان الكارت</dd>
        <dt>
            <input
                type="text"
                class="border-2 w-3/6 rounded-lg p-2 text-center"
                on:input:target=move |ev| {
                    let value =ev.target().value();
                    title.set(value.trim().to_string());
                }
            />
        </dt>
    }
}

#[component]
fn ColumnsIndexs(indexs: RwSignal<Vec<usize>>) -> impl IntoView {
    let style = RwSignal::new("");
    view! {
        <dd class="text-2xl m-2 p-2 font-bold border-l-2 border-r-2 rounded-xl">مسلسلات الاعمدة</dd>
        <dt>
            <input
                type="text"
                style=style
                class="border-2 w-4/6 rounded-lg p-2 text-center"
                on:input:target=move |ev| {
                    let value =ev.target().value();
                    let value = value.split(',').map(|x| x.trim().parse::<usize>()).collect::<Vec<_>>();
                    if value.iter().any(|x| x.is_err()) {
                        style.set("color:red");
                    } else {
                        indexs.set(value.iter().flatten().cloned().collect());
                        style.set("");
                    };
                }
            />
        </dt>
    }
}

#[component]
fn SheetName(sheet: RwSignal<String>) -> impl IntoView {
    view! {
        <dd class="text-2xl m-2 p-2 font-bold border-l-2 border-r-2 rounded-xl">اسم الشييت</dd>
        <dt>
            <input
                type="text"
                class="border-2 w-3/6 rounded-lg p-2 text-center"
                on:input:target=move |ev| {
                    let value =ev.target().value();
                    if !value.is_empty() {
                        sheet.set(value.trim().to_string());
                    }
                }
            />
        </dt>
    }
}

#[component]
fn TitleRowIndex(index: RwSignal<Option<NonZeroUsize>>) -> impl IntoView {
    let valid = RwSignal::new(true);
    let style = move || {
        if valid.get() {
            return "";
        };
        "color:red;"
    };
    view! {
        <dd class="text-2xl m-2 p-2 font-bold border-l-2 border-r-2 rounded-xl">مسلسل صف العناوين</dd>
        <dt>
            <input
                type="text"
                style=style
                class="border-2 w-2/6 rounded-lg p-2 text-center"
                placeholder="1"
                on:input:target=move |ev| {
                    let value =ev.target().value().parse::<NonZeroUsize>();
                    match value {
                        Ok(value) => {
                             valid.set(true);
                             index.set(Some(value))
                        },
                        Err(err) => {
                            valid.set(false);
                            log!("Error : {err:#?}");
                        },
                    };
                }
            />
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

        let is_excel = input_path.extension().is_some_and(|x| {
            ["xls", "xlsx", "xlsm", "xlsb", "xla", "xlam"].contains(&x.to_str().unwrap_or(""))
        });

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
