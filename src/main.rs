use yew::prelude::*;
use gloo::timers::callback::Timeout;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="game-container">
            <WelcomeScreen />
        </div>
    }
}

#[function_component(WelcomeScreen)]
fn welcome_screen() -> Html {
    let show_text = use_state(|| false);
    let show_press_start = use_state(|| false);

    // Animación de texto que aparece después de 1 segundo
    {
        let show_text = show_text.clone();
        let show_press_start = show_press_start.clone();
        
        use_effect_with((), move |_| {
            let show_text = show_text.clone();
            let show_press_start = show_press_start.clone();
            
            Timeout::new(1000, move || {
                show_text.set(true);
            }).forget();

            Timeout::new(3000, move || {
                show_press_start.set(true);
            }).forget();

            || {}
        });
    }

    html! {
        <div class="welcome-screen">
            <div class="stars"></div>
            <div class="gradient-bg"></div>
            
            <div class="content">
                <h1 class={classes!("game-title", (*show_text).then(|| "fade-in"))}>
                    {"RETRO QUEST"}
                </h1>
                
                <div class={classes!("subtitle", (*show_text).then(|| "slide-up"))}>
                    {"~ Una Aventura Épica ~"}
                </div>

                if *show_press_start {
                    <div class="press-start">
                        {"PRESIONA START"}
                    </div>
                }
            </div>

            <div class="pixel-border"></div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}