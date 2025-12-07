use crate::app::cards::CardsServerProps;
use leptos::logging::log;
use leptos::prelude::*;
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
        <dl>
            <CardTitle title/>
            <TitleRowIndex index=title_row_index/>
            <XlsxPath path/>
            <SheetName sheet/>
            <ColumnsIndexs indexs=columns_indexs/>
            <button on:click=on_submit>تمام</button>
        </dl>
    }
}

#[component]
fn CardTitle(title: RwSignal<String>) -> impl IntoView {
    view! {
        <dd>عنوان الكارت</dd>
        <dt>
            <input
                type="text"
                class="border-2"
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
        <dd>مسلسلات الاعمدة</dd>
        <dt>
            <input
                type="text"
                style=style
                class="border-2"
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
        <dd>اسم الشييت</dd>
        <dt>
            <input
                type="text"
                class="border-2"
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
        <dd>مسلسل صف العناوين</dd>
        <dt>
            <input
                type="text"
                style=style
                class="border-2"
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

#[server]
async fn path_exists(path: PathBuf) -> Result<bool, ServerFnError> {
    Ok(path.exists())
}

#[component]
fn XlsxPath(path: RwSignal<Option<PathBuf>>) -> impl IntoView {
    let input_path = RwSignal::new(PathBuf::new());
    let style = RwSignal::new("");

    let valid = Resource::new(move || input_path.get(), path_exists);
    Effect::new(move || {
        let valid = valid.get().transpose().ok().flatten().unwrap_or(false);
        if valid {
            path.set(Some(input_path.get_untracked()));
            style.set("");
        } else {
            style.set("color:red;");
        }
    });

    view! {
        <dd>موقع ملف الاكسل</dd>
        <dt>
            <input
                dir="ltr"
                type="text"
                class="border-2"
                style=style
                on:input:target=move |ev| {
                    let value =ev.target().value().parse::<PathBuf>();
                    let Ok(value) = value;
                    input_path.set(value);
                }
            />
        </dt>
    }
}
