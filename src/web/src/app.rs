use clap::Parser;
use yew::prelude::*;
use yew_router::prelude::*;
use yew_bootstrap::{util::{include_cdn, include_cdn_js, Color}, component::Spinner};

use crate::{
    components::layout::Layout,
    services::{config::ConfigProvider, alert::provider::AlertProvider, auth::AuthProvider},
    route::{Route, switch},
    opts::Opts,
};


#[function_component(App)]
pub fn app() -> Html {

    let opts = Opts::parse();

    let fallback = html! {<div style="position: absolute; top: 50%; left: 50%;"><Spinner style={Color::Warning} /></div>};

    html! {
        <>
            {include_cdn()}
            <Suspense {fallback}>
                <ConfigProvider<Opts> config={opts}>
                    <AlertProvider>
                        <AuthProvider>
                            <BrowserRouter>
                                <Layout>
                                    <Switch<Route> render={switch} />
                                </Layout>
                            </BrowserRouter>
                        </AuthProvider>
                    </AlertProvider>
                </ConfigProvider<Opts>>
            </ Suspense>
            {include_cdn_js()}
        </>
    }
}
