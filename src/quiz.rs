use topcoat::{
    Result,
    context::Cx,
    router::page,
    runtime::{Event, shard, signal},
    view::{View, attributes, class, component, error_boundary, suspense, view},
};

mod api;
pub mod model;

use crate::components::{
    alert::{AlertVariant, alert, alert_description, alert_title},
    button::{ButtonSize, ButtonVariant, button_variants},
    card::{card, card_content, card_footer, card_header},
    label::label,
    radio_group::{radio_group, radio_group_item},
    spinner::spinner,
};

#[page("/quiz")]
async fn quiz() -> Result<impl View> {
    let session: String = format!("{:x}", rand::random::<u128>());
    Ok(view! {
        <main
            class=(class!("flex flex-1 w-full items-center justify-center px-4 py-12"))
        >
            <div class=(class!("w-full max-w-2xl"))>
                error_boundary(
                    fallback: |error| Ok(
                            view! {
                                alert(
                                    variant: AlertVariant::Destructive,
                                    alert_title("Unable to load quiz")
                                    alert_description((error.to_string()))
                                )
                            },
                        ),
                    suspense(
                        fallback: view! {
                            <div
                                class=(class!("flex min-h-64 items-center justify-center"))
                            >
                                spinner(size: topcoat::view::Length::px(48.0))
                            </div>
                        },
                        question(
                            difficulty: $("facile".to_owned()),
                            session: $(session)
                        )
                    )
                )
            </div>
        </main>
    })
}

#[shard]
async fn question(cx: &Cx, difficulty: String, session: String) -> Result<impl View> {
    let index = signal(cx, || 0usize);
    let selected = signal(cx, || -1isize);
    let valided = signal(cx, || false);
    let score = signal(cx, || 0usize);
    let questions = api::fetch_questions(cx, &difficulty, &session).await?;

    let q = questions
        .get((index.get() as usize).min(questions.len().saturating_sub(1)))
        .unwrap()
        .clone();
    let answer_index = q.answer as isize;

    Ok(view! {
        if index.get() >= questions.len() {
            result(score: score.get(), total: questions.len())
        } else {
            card(
                card_header(
                    <div class=(class!("flex flex-col gap-2"))>
                        <h1
                            class=(class!("text-2xl font-semibold tracking-tight sm:text-3xl"))
                        >
                            (q.question)
                        </h1>
                    </div>
                )
                card_content(
                    radio_group(
                        #[key(i)]
                        for (i, a) in q
                            .answers
                            .iter()
                            .enumerate()
                            .map(|(i, a)| (i as isize, a)) {
                            label(
                                attrs: attributes! {
                                    for=(format!("answer-{i}"))
                                    class=(class!(
                                        "w-full cursor-pointer rounded-lg border border-border bg-background px-4 py-3 text-sm transition-colors",
                                        "hover:bg-foreground/5",
                                        "has-[:checked]:border-primary has-[:checked]:bg-primary/10",
                                        "!border-green-500 !bg-green-500/10" if valided.get()
                                            && i == q.answer as isize,
                                        "!border-red-500 !bg-red-500/10" if valided.get()
                                            && selected.get() == i
                                            && i != q.answer as isize,
                                        "pointer-events-none" if valided.get(),
                                    ))
                                },
                                radio_group_item(
                                    attrs: attributes! {
                                        id=(format!("answer-{i}"))
                                        name="answer"
                                        value=(i.to_string())
                                        @change=$(|_e: Event| selected.set(i))
                                    }
                                )
                                <span class=(class!("min-w-0 leading-relaxed"))>(a)</span>
                            )
                        }
                    )
                )
                card_footer(
                    attrs: attributes! { class="w-full" },
                    <a
                        href="/"
                        class=(class!(
                            button_variants(ButtonVariant::Secondary, ButtonSize::Md),
                            "w-1/3",
                        ))
                    >
                        "Home"
                    </a>
                    <button
                        type="button"
                        :disabled=$(selected.get() < 0isize)
                        @click=$(|_e| {
                            if valided.get() {
                                index.increment();
                                if selected.get() == answer_index {
                                    score.increment();
                                }
                                selected.set(-1isize);
                            }
                            valided.set(!valided.get());
                        })
                        class=(class!(
                            button_variants(ButtonVariant::Primary, ButtonSize::Md),
                            "flex-1",
                        ))
                    >
                        if index.get() >= questions.len() - 1 && valided.get() {
                            "See results"
                        } else if valided.get() {
                            "Next question"
                        } else {
                            "Validate"
                        }
                    </button>
                )
            )
        }
    })
}

#[component]
async fn result(score: usize, total: usize) -> Result<impl View> {
    let percentage = (score * 100).checked_div(total).unwrap_or(0);
    let message = if percentage >= 80 {
        "Excellent work!"
    } else if percentage >= 50 {
        "Good job!"
    } else {
        "Keep practicing!"
    };

    Ok(view! {
        card(
            card_header(
                <div class=(class!("flex flex-col items-center gap-2 text-center"))>
                    <p
                        class=(class!(
                            "text-sm font-medium uppercase tracking-widest text-muted-foreground",
                        ))
                    >
                        "Quiz complete"
                    </p>
                    <h2 class=(class!("text-2xl font-semibold tracking-tight"))>
                        (message)
                    </h2>
                </div>
            )
            card_content(
                <div
                    class=(class!(
                        "flex flex-col items-center gap-3 rounded-xl border border-border/60 bg-muted/30 px-6 py-8 text-center",
                    ))
                >
                    <span class=(class!("text-sm text-muted-foreground"))>
                        "Your score"
                    </span>
                    <div class=(class!("flex items-baseline gap-2"))>
                        <span class=(class!("text-5xl font-semibold tracking-tight"))>
                            (score)
                        </span>
                        <span class=(class!("text-xl text-muted-foreground"))>
                            "/ "
                            (total)
                        </span>
                    </div>
                    <span class=(class!("text-sm font-medium text-primary"))>
                        (format!("{percentage:.0}%"))
                    </span>
                </div>
            )
            card_footer(
                attrs: attributes! { class="w-full" },
                <a
                    href="/"
                    class=(class!(
                        button_variants(ButtonVariant::Secondary, ButtonSize::Md),
                        "flex-1",
                    ))
                >
                    "Home"
                </a>
                <a
                    href="/quiz"
                    class=(class!(
                        button_variants(ButtonVariant::Primary, ButtonSize::Md),
                        "flex-1",
                    ))
                >
                    "Start a new quiz"
                </a>
            )
        )
    })
}
