#![recursion_limit = "128"]

extern crate stdweb;
#[macro_use]
extern crate yew;
extern crate meval;

use yew::prelude::*;
use yew::services::console::ConsoleService;

//ugly hack, tesing yew library (writing front-end web apps with rust)

//using macro is really ugly but was too lazy to write whole component for single button
macro_rules! number_btn {
    ($number:expr) => {
        html! { <button class="button", class="is-primary", class="column",class="small-margin", onclick=|_| Msg::OnNumberButton($number) ,>{$number }</button> }
    };
}
macro_rules! op_btn {
    ($op:expr) => {
        html! { <button class="button", class="column",class="small-margin", onclick=|_| Msg::OnOperatorButton($op.to_string()) ,>{$op }</button> }
    };
}

pub struct Model {
    expression: String,
}

pub enum Msg {
    OnNumberButton(i32),
    OnOperatorButton(String),
    OnEvaluationButton,
    OnResetButton,
}

impl<CTX> Component<CTX> for Model
where
    CTX: AsMut<ConsoleService>,
{
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        Model {
            expression: "".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message, _env: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            Msg::OnNumberButton(number) => self.expression.push_str(&number.to_string()),
            Msg::OnOperatorButton(operator) => self.expression.push_str(&operator),
            Msg::OnEvaluationButton => {
                let r = meval::eval_str(&self.expression);
                match r
                {
                    Ok(result) => self.expression = result.to_string(),
                    Err(_) => self.expression = "invalid math expression".to_string()
                }
            }
            Msg::OnResetButton => {
                self.expression = "".to_string();
            }
        }
        true
    }
}

impl<CTX> Renderable<CTX, Model> for Model
where
    CTX: AsMut<ConsoleService> + 'static,
{
    fn view(&self) -> Html<CTX, Self> {
        html! {
          <section class="section",>
          <div class="container", class="narrow",>
                <div class="field",>
                    <div class="control",>
                        <input class="label", class="is-fullwidth",value={&self.expression},/>
                    </div>
                </div>

                //<p class="label",>{ &self.expression }</p>
                <div class="columns",>{number_btn!(1)}{number_btn!(2)}{number_btn!(3)}</div>
                <div class="columns",>{number_btn!(4)}{number_btn!(5)}{number_btn!(6)}</div>
                <div class="columns",>{number_btn!(7)}{number_btn!(8)}{number_btn!(9)}</div>
                <div class="columns",> {number_btn!(0)}{op_btn!("-")}{op_btn!("+")}</div>
                <div class="columns",> {op_btn!("*")}{op_btn!("/")}
                <button class="button ", class="column",class="small-margin", onclick=|_| Msg::OnEvaluationButton,>{ "=" }</button>
                </div>
                <div class="columns",>

                <button class="button", class="is-fullwidth", class="is-danger", onclick=|_| Msg::OnResetButton,>{ "Reset" }</button>
                </div>
          </div>
          </section>

        }
    }
}
