use crate::{
    components::{
        button::{ButtonSize, ButtonVariant, button_variants},
        card::{card, card_content, card_description, card_header, card_title},
        field::{field, field_content, field_description, field_label},
        label::label,
        radio_group::{radio_group, radio_group_item},
    },
    quiz::{self, DEFAULT_QUESTIONS, DIFFICULTIES, MAX_QUESTIONS, MIN_QUESTIONS},
};
use topcoat::{
    self, Result,
    asset::{AssetBundle, RouterBuilderAssetExt},
    font::fontsource::fontsource_font,
    router::{Router, RouterBuilderDiscoverExt, Slot, layout, page},
    runtime::RouterBuilderRuntimeExt,
    tailwind,
    view::{View, attributes, class, component, view},
};

pub fn router() -> topcoat::router::Router {
    Router::builder()
        .discover()
        .assets(AssetBundle::load().unwrap())
        .runtime()
        .app_context(quiz::model::QuizCache::new())
        .build()
}

#[page("/")]
async fn home() -> Result<impl View> {
    Ok(view! {
        <main class=(class!("flex flex-1 items-center justify-center px-4 py-12"))>
            <div
                class=(class!(
                    "flex w-full max-w-2xl flex-col items-center gap-6 text-center",
                ))
            >
                <h1 class=(class!("text-3xl font-semibold tracking-tight sm:text-4xl"))>
                    "Quiz"
                </h1>
                settings_form()
            </div>
        </main>
    })
}

#[component]
async fn settings_form() -> Result<impl View> {
    Ok(view! {
        card(
            attrs: attributes! { class="w-full text-left" },
            card_header(
                card_title("Quiz settings")
                card_description("Pick a difficulty and a number of questions.")
            )
            card_content(
                <form method="get" action="/quiz" class=(class!("flex flex-col gap-6"))>
                    <div class=(class!("flex flex-col gap-3"))>
                        <span class=(class!("text-sm font-medium"))>"Difficulty"</span>
                        radio_group(
                            #[key(i)]
                            for (i, value) in DIFFICULTIES.into_iter().enumerate() {
                                label(
                                    attrs: attributes! {
                                        for=(format!("difficulty-{i}"))
                                        class=(class!(
                                            "w-full cursor-pointer rounded-lg border border-border bg-background px-4 py-3 text-sm transition-colors",
                                            "hover:bg-foreground/5",
                                            "has-[:checked]:border-primary has-[:checked]:bg-primary/10",
                                        ))
                                    },
                                    radio_group_item(
                                        attrs: attributes! {
                                            id=(format!("difficulty-{i}"))
                                            name="difficulty"
                                            value=(value)
                                            if i == 0 {
                                                checked="checked"
                                            }
                                        }
                                    )
                                    <span class=(class!("min-w-0 capitalize leading-relaxed"))>
                                        (value)
                                    </span>
                                )
                            }
                        )
                    </div>
                    field(
                        field_label(
                            attrs: attributes! { for="limit" },
                            "Number of questions"
                        )
                        field_content(
                            <input
                                id="limit"
                                name="limit"
                                type="number"
                                min=(MIN_QUESTIONS.to_string())
                                max=(MAX_QUESTIONS.to_string())
                                value=(DEFAULT_QUESTIONS.to_string())
                                aria-describedby="limit-description"
                                class=(class!(
                                    "h-10 w-full rounded-lg border border-border bg-background px-3 py-2 text-sm outline-none transition-colors placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50 focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background",
                                ))
                            />
                            field_description(
                                attrs: attributes! { id="limit-description" },
                                (format!(
                                    "Choose between {MIN_QUESTIONS} and {MAX_QUESTIONS} questions.",
                                ))
                            )
                        )
                    )
                    <button
                        type="submit"
                        class=(class!(
                            button_variants(ButtonVariant::Primary, ButtonSize::Md),
                            "w-full",
                        ))
                    >
                        "Start quiz"
                    </button>
                </form>
            )
        )
    })
}

#[layout("/")]
async fn root_layout(slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <title>"Quiz"</title>
                <meta
                    name="description"
                    content="Test your knowledge with a short quiz."
                />
                topcoat::font::link(font: fontsource_font!(GEIST))
                <link rel="stylesheet" href=(tailwind::stylesheet!())>
                topcoat::dev::script()
                topcoat::runtime::script()
            </head>
            <body class=(class!("flex min-h-screen flex-col"))>(slot)</body>
        </html>
    })
}
