use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <div class="panel">
                <div class="formula-container">
                    <label for="formula">{ "Formula" }</label>
                    <input id="formula" type="text" class="control control-lg" required=true />
                </div>
                <div class="inputs-container">
                    <label for="start">{ "Range of inputs" }</label>
                    <input id="start" type="number" class="control control-sm" required=true />
                    <input id="end" type="number" class="control control-sm" required=true />
                </div>
                <div>
                    <button class="btn btn-sm">{ "Go!" }</button>
                </div>
            </div>
            <div class="canvas">
                <svg width=600 height=600>
                    <g stroke="#007bff">
                        <path d="M 0 0 L 100 100"></path>
                    </g>
                </svg>
            </div>
        </main>
    }
}
