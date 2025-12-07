use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use std::{num::NonZeroUsize, path::PathBuf};

#[component]
pub fn Cards(title: String, csp: CardsServerProps) -> impl IntoView {
    let cards = Resource::new(move || csp.clone(), get_cards);
    let cardsfn = move || cards.get().transpose().ok().flatten().unwrap_or_default();

    view! {
        <Transition>
        <div class="grid grid-cols-3 gap-5">
            <For
                each=cardsfn
                key=|x| x.row_index
                let(Card { row_index:_, kv })
                >
                    <div class="border-sky-500 border-5 rounded-xl p-2 m-2 text-xl text-center">
                        <h2 class="font-bold">{title.clone()}</h2>
                        <dl class="divide-y divide-white/10">
                            <For
                                each=move || kv.clone()
                                key=|x| x.key.clone()
                                let(Kv { key, value })
                            >
                                 <div class="flex">
                                    <dt class="px-3 border-l-2 border-dotted">{key}</dt>
                                    <dd class="grow">{value}</dd>
                                </div>
                            </For>
                        </dl>
                    </div>
            </For>
        </div>
        </Transition>
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Card {
    pub row_index: usize,
    pub kv: Vec<Kv>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Kv {
    pub key: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CardsServerProps {
    pub title_row_index: Option<NonZeroUsize>,
    pub path: PathBuf,
    pub sheet: String,
    pub columns_indexes: Vec<usize>,
}

#[server]
async fn get_cards(reqs: CardsServerProps) -> Result<Vec<Card>, ServerFnError> {
    let CardsServerProps {
        title_row_index,
        path,
        sheet,
        columns_indexes,
    } = reqs;
    use calamine::{open_workbook, Data, DeError, RangeDeserializerBuilder, Reader, Xlsx};

    let mut workbook: Xlsx<_> = open_workbook(&path)?;
    let range = workbook.worksheet_range(&sheet)?;

    let mut iter = RangeDeserializerBuilder::new()
        .has_headers(false)
        .from_range(&range)?;

    let headers = match title_row_index {
        Some(i) => iter.nth(Into::<usize>::into(i) - 1),
        None => iter.next(),
    }
    .unwrap_or(Err(DeError::HeaderNotFound(String::from(
        "Error : first row should contain headers",
    ))))?;

    let mut cards = Vec::new();
    for (i, row) in iter.enumerate() {
        let mut kvs = Vec::new();
        let row: Vec<Data> = row?;
        for index in columns_indexes.iter() {
            let header = headers[*index].to_string();
            let value = row[*index].to_string();
            if !header.is_empty() && !value.is_empty() {
                kvs.push(Kv {
                    key: header,
                    value: value,
                });
            }
        }
        cards.push(Card {
            row_index: i,
            kv: kvs,
        });
    }

    Ok(cards)
}
