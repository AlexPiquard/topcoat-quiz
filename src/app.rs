use crate::{
    components::button::{ButtonSize, ButtonVariant, button_variants},
    quiz,
};
use topcoat::{
    self, Result,
    asset::{AssetBundle, RouterBuilderAssetExt},
    font::fontsource::fontsource_font,
    router::{Router, RouterBuilderDiscoverExt, Slot, layout, page},
    runtime::RouterBuilderRuntimeExt,
    tailwind,
    view::{View, class, view},
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
                <a
                    href="/quiz"
                    class=(button_variants(ButtonVariant::Primary, ButtonSize::Lg))
                >
                    "Start new quiz"
                </a>
            </div>
        </main>
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
