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
    let show_content = use_state(|| false);

    // Animación de aparición del contenido
    {
        let show_content = show_content.clone();
        
        use_effect_with((), move |_| {
            let show_content = show_content.clone();
            
            Timeout::new(500, move || {
                show_content.set(true);
            }).forget();

            || {}
        });
    }

    html! {
        <div class="welcome-screen">
            <div class="stars"></div>
            
            <Particles />
            
            <Snorlax />
            
            <div class={classes!("content", (*show_content).then(|| "show"))}>
                <h1 class="game-title">{"RETRO QUEST"}</h1>
                <div class="subtitle">{"~ Una Aventura Épica ~"}</div>
                <div class="press-start">{"PRESIONA START"}</div>
            </div>
            
            <div class="retro-border"></div>
        </div>
    }
}

//#[function_component(Snorlax)]
/*fn snorlax() -> Html {
    html! {
        <div class="snorlax">
            <div class="snorlax-body">
                <div class="snorlax-belly"></div>
                <div class="snorlax-head">
                    <div class="snorlax-eyes">
                        <div class="snorlax-eye"></div>
                        <div class="snorlax-eye"></div>
                    </div>
                </div>
                <div class="snorlax-arms">
                    <div class="snorlax-arm left"></div>
                    <div class="snorlax-arm right"></div>
                </div>
            </div>
        </div>
    }
}*/

#[function_component(Particles)]
fn particles() -> Html {
    html! {
        <div class="particles">
            <div class="particle particle-1"></div>
            <div class="particle particle-2"></div>
            <div class="particle particle-3"></div>
            <div class="particle particle-4"></div>
            <div class="particle particle-5"></div>
            <div class="particle particle-6"></div>
            <div class="particle particle-7"></div>
            <div class="particle particle-8"></div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}