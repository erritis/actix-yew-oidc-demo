
use yew::{Properties, Children, function_component, Html, html, ContextProvider};

use yew::{hook, use_context};

#[hook]
pub fn use_config<T: Clone + PartialEq + 'static>() -> T {
    use_context::<T>().unwrap()
}


/// Props for [`ConfigProvider`]
#[derive(Properties, Clone, PartialEq)]
pub struct ConfigProviderProps<Config: Clone + PartialEq + 'static> {
    pub children: Children,
    pub config: Config,
}

/// The config provider component.
///
/// Every child (direct or indirect) of this component can use `use_config` hook to get config info.
/// `Config` - type of the config object.
///
/// # Example
///
/// ```
///
/// html! {
///     <ConfigProvider<CustomConfig> {config}>
///         <MyComponent />
///     </ConfigProvider<CustomConfig>>
/// }
/// 
/// ```
#[function_component(ConfigProvider)]
pub fn config_provider<
    Config: Clone + PartialEq + 'static,
>(
    props: &ConfigProviderProps<Config>,
) -> Html {

    let config = props.config.clone();
    let children = props.children.clone();

    html! {
        <ContextProvider<Config> context={config}>
            {children}
        </ContextProvider<Config>>
    }
}
