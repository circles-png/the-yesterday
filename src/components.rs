use crate::crossword::{Crossword, Direction, Vec2};
use itertools::Itertools;
use leptos::ev::{keydown, KeyboardEvent, MouseEvent};
use leptos::leptos_dom::helpers::location;
use leptos::web_sys::{HtmlButtonElement, HtmlInputElement};
use leptos_meta::{provide_meta_context, Meta};
use std::collections::HashMap;
use std::iter::once;
use std::ops::{Index, Neg, Not};
use std::str::FromStr;

use crate::ad::ADS;
use crate::article::{Article, ARTICLES};
use crate::article::{Fragment, Image};
use crate::crossword::CROSSWORDS;
use chrono::Local;

use leptos::{
    component, create_memo, create_signal, event_target, view, window_event_listener, Callback,
    Children, CollectView, IntoView, Params, SignalGet, SignalWith,
};
use leptos_router::A;
use leptos_router::{use_params, Route, Router, Routes};
use leptos_router::{use_params_map, Params};
use rand::seq::SliceRandom;
use rand::thread_rng;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Router>
            <div class="flex flex-col h-full">
                <Header/>
                <PageContainer>
                    <Routes>
                        <Route path="/" view=ArticlePreviews/>
                        <Route path="/articles/:id" view=Article/>
                        <Route path="/crosswords/:id" view=Crossword/>
                        <Route path="/*" view=|| "404"/>
                    </Routes>
                </PageContainer>
                <Footer/>
            </div>
        </Router>
    }
}

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="relative p-4 text-white bg-black">
            <div class="inset-0 items-center justify-between hidden pointer-events-none sm:p-4 sm:absolute sm:flex">
                <div>{Local::now().format("%B %-d, %Y").to_string()}</div>
                <A
                    href="https://angusmason.github.io/theaccountgame"
                    target="_blank"
                    class="pointer-events-auto"
                >
                    "Sign Up"
                </A>
            </div>
            <a
                class="w-full text-center"
                href="/"
                on:click=|_| {
                    location().reload().unwrap();
                }
            >

                <Heading>
                    <div class="text-5xl capitalize font-blackletter">"The Yesterday"</div>
                    <div class="block font-serif text-base">"Trusted by dozens."</div>
                </Heading>
            </a>
        </header>
    }
}

#[component]
pub fn PageContainer(children: Children) -> impl IntoView {
    view! {
        <main class="flex justify-center gap-4 p-4 grow">
            <div class="w-full max-w-2xl shrink-0">{children()}</div>
        </main>
    }
}

#[component]
pub fn ArticlePreviews() -> impl IntoView {
    const ALL: &str = "Archive";
    let (filter, set_filter) = create_signal(None::<&str>);
    view! {
        <Meta
            name="description"
            content="Australia's most serious newspaper, proudly brought to you by incredible (and a few credible) reporters."
        />
        <div class="flex flex-col gap-2">
            <div class="hidden md:flex *:px-2 divide-x font-serif justify-center">

                {move || {
                    ARTICLES
                        .iter()
                        .map(|article| article.topic)
                        .chain(once(ALL))
                        .unique()
                        .map(|topic| {
                            view! {
                                <button
                                    class=filter
                                        .get()
                                        .as_ref()
                                        .map_or(false, |filter| topic == *filter)
                                        .then_some("text-blue-800")

                                    on:click=move |_| set_filter(Some(topic))
                                >

                                    {topic}
                                </button>
                            }
                        })
                        .collect_view()
                }}

            </div>
            <div class="flex flex-col gap-2">
                {move || {
                    const LATEST: &str = "Latest";
                    once(LATEST)
                        .chain(ARTICLES.iter().map(|article| article.topic).unique())
                        .chain(once(ALL))
                        .filter(|topic| {
                            filter.get().as_ref().map_or(*topic != ALL, |filter| topic == filter)
                        })
                        .map(|topic| {
                            view! {
                                <Heading>{topic}</Heading>
                                <Divider/>
                                <div class="flex flex-col grid-cols-2 gap-8 sm:grid">
                                    {move || {
                                        match topic {
                                            LATEST => ARTICLES.iter().take(6).cloned().collect_vec(),
                                            ALL => ARTICLES.iter().cloned().collect_vec(),
                                            _ => {
                                                ARTICLES
                                                    .iter()
                                                    .filter(|article| article.topic == topic)
                                                    .cloned()
                                                    .collect_vec()
                                            }
                                        }
                                            .iter()
                                            .map(|article| {
                                                view! { <ArticlePreview article=article.clone()/> }
                                            })
                                            .collect_view()
                                    }}

                                </div>
                            }
                        })
                        .collect_view()
                }}

            </div>
        </div>
    }
}

#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn ArticlePreview(
    article: Article,
    #[prop(optional)] no_blurb: bool,
    #[prop(optional)] horizontal: bool,
) -> impl IntoView {
    view! {
        <A
            class=format!(
                "flex gap-3 {}",
                horizontal.not().then_some("flex-col").unwrap_or_default(),
            )

            href=format!("/articles/{}", article.id)
        >
            <img
                src=article.image.url
                alt=article.image.caption
                class="object-cover w-full aspect-[3/2]"
            />
            <div>
                <small class="text-sm font-light text-blue-800">
                    {article.topic.to_uppercase()}
                </small>
                <Heading>
                    <article class="text-xl">{article.title}</article>
                </Heading>
                {no_blurb
                    .not()
                    .then_some(
                        view! {
                            <Caption>
                                <div class="text-sm text-left">{article.blurb}</div>
                            </Caption>
                        },
                    )}

            </div>
        </A>
    }
}

#[component]
pub fn Article() -> impl IntoView {
    #[derive(Params, PartialEq)]
    struct ArticleParams {
        id: String,
    }
    let article = || {
        use_params::<ArticleParams>().with(|params| {
            ARTICLES
                .iter()
                .find(|article| article.id == params.as_ref().unwrap().id.clone())
                .unwrap()
        })
    };
    view! {
        <Meta name="description" content=article().blurb/>
        <div class="flex flex-col gap-4">
            <div>
                <Heading>{move || article().title.to_uppercase()}</Heading>
                <Caption>
                    <div class="text-left sm:text-lg">{move || article().blurb}</div>
                </Caption>
                <div class="flex gap-1 text-sm font-light">
                    <div class="text-blue-800">{move || article().topic.to_uppercase()}</div>
                    "\u{b7} "
                    {move || article().reading_time()}
                    " min read"
                </div>
            </div>
            <div class="sm:px-16">
                <img
                    src=move || article().image.url
                    alt=move || article().image.caption
                    class="object-cover w-full aspect-[3/2]"
                />
                <Caption>{move || article().image.caption}</Caption>
            </div>
            <Divider/>
            <div class="flex flex-col gap-5 sm:text-lg
            [&>div:first-child>p]:first-letter:text-[2.8rem]
            sm:[&>div:first-child>p]:first-letter:text-[3.5rem]
            [&>div:first-child>p]:first-letter:leading-none
            [&>div:first-child>p]:first-letter:font-bold
            [&>div:first-child>p]:first-letter:font-serif
            [&>div:first-child>p]:first-letter:float-left
            [&>div:first-child>p]:first-letter:pr-2">
                {move || {
                    article()
                        .fragments
                        .iter()
                        .map(|fragment| {
                            match fragment {
                                Fragment::Image(Image { url, caption }) => {
                                    view! {
                                        <div class="px-16">
                                            <img src=*url alt=*caption class="object-cover w-full"/>
                                            <Caption>{*caption}</Caption>
                                        </div>
                                    }
                                }
                                Fragment::Text(text) => {
                                    view! {
                                        <div>
                                            <p>{*text}</p>
                                        </div>
                                    }
                                }
                            }
                        })
                        .collect_view()
                }}

            </div>
            <Divider/>
            <ReadMore this_article=article/>
        </div>
    }
}

#[component]
pub fn Divider(#[prop(optional)] light: bool) -> impl IntoView {
    view! {
        <div class=format!(
            "w-full h-px {}",
            if light { "bg-gray-200" } else { "bg-gray-800" },
        )></div>
    }
}

#[component]
pub fn ReadMore(this_article: impl Fn() -> &'static Article + 'static) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4">
            <Heading>"Read More"</Heading>
            <div class="flex flex-col w-full gap-4 [&_img]:w-1/4">
                {move || {
                    let mut articles = ARTICLES.to_vec();
                    articles.shuffle(&mut thread_rng());
                    let same_topic = articles
                        .iter()
                        .filter(|article| {
                            **article != *this_article() && article.topic == this_article().topic
                        })
                        .collect_vec();
                    let selected = if same_topic.is_empty() {
                        articles.iter().filter(|article| **article != *this_article()).collect_vec()
                    } else {
                        same_topic
                    };
                    selected
                        .into_iter()
                        .take(3)
                        .map(|article| {
                            view! {
                                <ArticlePreview
                                    article=article.clone()
                                    horizontal=true
                                    no_blurb=true
                                />
                            }
                        })
                        .collect_view()
                }}

            </div>
        </div>
    }
}

#[component]
pub fn Heading(children: Children) -> impl IntoView {
    view! { <h1 class="font-serif text-2xl font-medium uppercase sm:text-4xl">{children()}</h1> }
}

#[component]
pub fn Footer() -> impl IntoView {
    let ad = ADS.choose(&mut thread_rng()).unwrap();
    let (show_overlay, set_show_overlay) = create_signal(false);
    view! {
        <footer class="flex flex-col p-4 text-white bg-black">
            <A href="/">
                <Heading>
                    <div class="capitalize font-blackletter">"The Yesterday"</div>
                </Heading>
            </A>
            <div class="flex justify-between">
                <div>"Copyright \u{a9} 2024"</div>
                "Brought to you by incredible (and a few credible) reporters."
            </div>
        </footer>
        <div class="sticky bottom-0 flex justify-center w-full p-2 bg-gray-100 border">
            <div class="relative">
                <div class="relative">
                    <img src=format!("/images/horizontal-ads/{}", *ad) class="h-24 cursor-pointer"/>
                    <div class=move || {
                        format!(
                            "absolute inset-0 z-10 flex flex-col items-center gap-1 p-2 bg-gray-100 border text-neutral-500 {}",
                            if show_overlay.get().not() {
                                "opacity-0 pointer-events-none"
                            } else {
                                "opacity-100 transition-opacity duration-1000"
                            },
                        )
                    }>
                        <button
                            class="absolute top-0 left-0 p-2 text-2xl leading-none"
                            on:click=move |_| set_show_overlay(false)
                        >
                            "\u{2190}"
                        </button>
                        <h1 class="text-sm">
                            "Ads not by " <span class="font-bold">"Google"</span>
                        </h1>
                        <div class="flex flex-col w-full gap-1 px-16 text-xs">
                            <button
                                class="w-full py-1 text-white bg-blue-500 rounded-sm shadow"
                                on:click=move |_| set_show_overlay(false)
                            >
                                "Keep seeing this ad"
                            </button>
                            <button
                                class="w-full py-1 bg-white rounded-sm shadow"
                                on:click=move |_| set_show_overlay(false)
                            >
                                "Why not this ad? \u{25B7}"
                            </button>
                        </div>
                    </div>
                </div>
                <div class="text-sm text-center opacity-50">"Advertisement"</div>
                <button
                    class="absolute top-0 right-0 flex text-xs leading-none text-blue-500"
                    on:click=move |_| set_show_overlay(true)
                >
                    <div class="grid border bg-gray-100/50 size-4 place-content-center">
                        <div class="cursor-pointer border rounded-full text-[8px] aspect-square size-3 grid place-content-center border-blue-500 font-medium">
                            i
                        </div>
                    </div>
                    <div class="grid border place-content-center bg-gray-100/50 size-4">"X"</div>
                </button>
            </div>
        </div>
    }
}

#[component]
pub fn Caption(children: Children) -> impl IntoView {
    view! { <caption class="block w-full py-2 text-sm text-right opacity-50">{children()}</caption> }
}

#[component]
#[allow(clippy::needless_lifetimes, clippy::too_many_lines)]
pub fn CrosswordGrid(
    grid: Vec<Option<(char, Option<usize>)>>,
    crossword: &'static Crossword,
    #[prop(into)] on_solution_change: Callback<HashMap<usize, Option<char>>>,
) -> impl IntoView {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Move {
        Left,
        Right,
        Up,
        Down,
        Next,
        Previous,
    }
    impl Move {
        const fn direction(self) -> Option<Direction> {
            match self {
                Self::Left | Self::Right => Some(Direction::Across),
                Self::Up | Self::Down => Some(Direction::Down),
                Self::Next | Self::Previous => None,
            }
        }
        const fn new_index(self, crossword_size: Vec2, selected: usize) -> Option<usize> {
            match self {
                Self::Left => Some(selected - 1),
                Self::Right => Some(selected + 1),
                Self::Up => Some(selected - (crossword_size.x)),
                Self::Down => Some(selected + crossword_size.x),
                Self::Next | Self::Previous => None,
            }
        }
        const fn out_of_bounds(self, crossword_size: Vec2, selected: usize) -> Option<bool> {
            match self {
                Self::Left => Some(selected % crossword_size.x == 0),
                Self::Right => Some(selected % crossword_size.x == crossword_size.x - 1),
                Self::Up => Some(selected < crossword_size.x),
                Self::Down => Some(selected >= crossword_size.x * (crossword_size.y - 1)),
                Self::Next | Self::Previous => None,
            }
        }
        const fn from_direction(direction: Direction) -> Self {
            match direction {
                Direction::Across => Self::Right,
                Direction::Down => Self::Down,
            }
        }
    }
    impl Neg for Move {
        type Output = Self;
        fn neg(self) -> Self::Output {
            match self {
                Self::Left => Self::Right,
                Self::Right => Self::Left,
                Self::Up => Self::Down,
                Self::Down => Self::Up,
                Self::Next => Self::Previous,
                Self::Previous => Self::Next,
            }
        }
    }
    enum SetSolution {
        Clear,
        Write(char),
        Keep,
    }
    let (selected, set_selected) = create_signal(None::<usize>);
    let (solution, set_solution) = create_signal(
        grid.iter()
            .enumerate()
            .filter_map(|(index, cell)| cell.as_ref().map(|_| (index, None::<char>)))
            .collect::<HashMap<_, _>>(),
    );
    let (last_direction, set_last_direction) = create_signal(None::<Direction>);
    let size = crossword.size();
    {
        let grid = grid.clone();
        let handler = move |event: KeyboardEvent| {
            let (new, movement) = match event.key().as_str() {
                key if key.len() == 1 && key.chars().next().unwrap().is_ascii_alphabetic() => (
                    SetSolution::Write(key.chars().next().unwrap().to_ascii_uppercase()),
                    Move::Next,
                ),
                "ArrowLeft" => (SetSolution::Keep, Move::Left),
                "ArrowRight" => (SetSolution::Keep, Move::Right),
                "ArrowUp" => (SetSolution::Keep, Move::Up),
                "ArrowDown" => (SetSolution::Keep, Move::Down),
                "Backspace" => (SetSolution::Clear, Move::Previous),
                "Escape" => {
                    set_selected(None);
                    event.prevent_default();
                    return;
                }
                _ => return,
            };
            event.prevent_default();
            let Some(selected) = selected.get() else {
                return;
            };
            'out: {
                let apply_move = |movement: Move| {
                    if movement.out_of_bounds(size, selected).unwrap() {
                        return None;
                    }
                    set_last_direction(movement.direction());
                    Some(movement.new_index(size, selected).unwrap())
                };
                let new_selected = match movement {
                    Move::Next | Move::Previous => {
                        let position = Vec2 {
                            x: selected % size.x,
                            y: selected / size.x,
                        };
                        let Some(word) = crossword
                            .words
                            .iter()
                            .find(|word| {
                                word.contains(position)
                                    && last_direction.with(|direction| {
                                        direction
                                            .map(|direction| word.direction == direction)
                                            .unwrap_or(true)
                                    })
                            })
                            .or_else(|| {
                                crossword.words.iter().find(|word| word.contains(position))
                            })
                        else {
                            break 'out;
                        };
                        match apply_move(match movement {
                            Move::Next => Move::from_direction(word.direction),
                            Move::Previous => -Move::from_direction(word.direction),
                            _ => unreachable!(),
                        }) {
                            None => break 'out,
                            Some(new_selected) => new_selected,
                        }
                    }
                    movement => match apply_move(movement) {
                        None => break 'out,
                        Some(new_selected) => new_selected,
                    },
                };
                if Option::is_none(grid.index(new_selected)) {
                    break 'out;
                };
                set_selected(Some(new_selected));
            }
            match new {
                SetSolution::Keep => {}
                new @ (SetSolution::Clear | SetSolution::Write(_)) => {
                    let mut solution = solution.get();
                    let char = solution.get_mut(&selected).unwrap();
                    *char = match new {
                        SetSolution::Clear => None,
                        SetSolution::Write(char) => Some(char),
                        SetSolution::Keep => unreachable!(),
                    };
                    on_solution_change(solution.clone());
                    set_solution(solution);
                }
            }
        };
        window_event_listener(keydown, handler);
    }
    view! {
        <div class="flex justify-center">
            <div
                class="grid overflow-x-auto"
                style=format!("grid-template-columns: repeat({}, auto);", size.x)
            >
                {grid
                    .into_iter()
                    .enumerate()
                    .map(|(index, cell)| {
                        cell.map_or_else(
                            || {
                                view! {
                                    <div class="bg-black">
                                        <button
                                            class="size-full"
                                            on:click=move |_| set_selected(None)
                                        ></button>
                                    </div>
                                }
                            },
                            |(_, word_start)| {
                                view! {
                                    <div class=move || {
                                        format!(
                                            "relative text-xl border border-black size-8 {}",
                                            (selected.get() == Some(index))
                                                .then_some("bg-yellow-200")
                                                .unwrap_or_default(),
                                        )
                                    }>
                                        <input
                                            class="text-center bg-transparent size-full focus:outline-none caret-transparent"
                                            on:focus=move |_| {
                                                match selected.get() {
                                                    Some(selected) if selected == index => set_selected(None),
                                                    _ => set_selected(Some(index)),
                                                }
                                            }
                                            value=move || {
                                                solution.get().get(&index).unwrap().unwrap_or_default()
                                            }
                                        />

                                        <div class="absolute text-[8px] leading-none opacity-50 inset-0.5 pointer-events-none">
                                            {word_start.map(|index| index + 1)}
                                        </div>
                                    </div>
                                }
                            },
                        )
                    })
                    .collect_view()}
            </div>
        </div>
    }
}

#[component]
#[allow(clippy::too_many_lines)]
pub fn Crossword() -> impl IntoView {
    let crossword = || -> &Crossword {
        &CROSSWORDS[use_params_map()
            .with(|params| <usize as FromStr>::from_str(params.get("id").unwrap()).unwrap())]
    };
    let starts = move || {
        crossword()
            .words
            .iter()
            .map(|word| word.position)
            .unique()
            .sorted_unstable_by(|a, b| a.y.cmp(&b.y).then_with(|| a.x.cmp(&b.x)))
            .collect_vec()
    };
    let grid = {
        move || {
            let size = crossword().size();
            (0..size.y)
                .flat_map(|y| {
                    (0..size.x)
                        .map(|x| {
                            crossword()
                                .to_letters()
                                .iter()
                                .find(|letter| letter.position == Vec2 { x, y })
                                .map(|letter| {
                                    (
                                        letter.character,
                                        starts().iter().position(|start| *start == Vec2 { x, y }),
                                    )
                                })
                        })
                        .collect_vec()
                })
                .collect_vec()
        }
    };
    let (solution, set_solution) = create_signal::<HashMap<usize, Option<char>>>(HashMap::new());
    let correct = create_memo(move |_| {
        !solution.get().is_empty()
            && solution
                .get()
                .iter()
                .all(|(index, letter)| match grid().get(*index).unwrap() {
                    None => true,
                    Some((char, _)) => letter == &Some(*char),
                })
    });
    let check = move |event: MouseEvent| {
        let button: HtmlButtonElement = event_target(&event);
        button.set_text_content(Some(format!("{:?}", correct()).as_str()));
    };
    view! {
        <div class="flex flex-col gap-2">

            {move || {
                view! {
                    <CrosswordGrid
                        grid=grid()
                        crossword=crossword()
                        on_solution_change=set_solution
                    />
                }
            }}
            <div class="flex justify-center">
                <button
                    class="px-4 py-2 text-white bg-black rounded disabled:opacity-50 disabled:pointer-events-none"
                    disabled=move || {
                        solution().is_empty()
                            || solution().iter().any(|(_, letter)| letter.is_none())
                    }

                    on:click=check
                >

                    "Check"
                </button>
            </div>
            <div class="flex flex-col grid-cols-2 gap-2 sm:grid">
                {move || {
                    Direction::ALL
                        .iter()
                        .map(|direction| {
                            view! {
                                <div class="flex flex-col">
                                    <h1 class="text-2xl font-semibold">{direction.to_string()}</h1>
                                    <div class="grid grid-cols-[auto_minmax(0,1fr)] gap-x-2">
                                        {crossword()
                                            .words
                                            .iter()
                                            .filter(|word| word.direction == *direction)
                                            .sorted_unstable_by_key(|word| {
                                                starts().iter().position(|start| *start == word.position)
                                            })
                                            .map(|word| {
                                                view! {
                                                    <div class="font-semibold">
                                                        {starts()
                                                            .iter()
                                                            .position(|start| *start == word.position)
                                                            .map(|index| index + 1)}
                                                    </div>
                                                    <div>{word.clue}</div>
                                                }
                                            })
                                            .collect_view()}
                                    </div>
                                </div>
                            }
                        })
                        .collect_view()
                }}

            </div>
        </div>
    }
}
