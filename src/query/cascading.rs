use async_graphql::dynamic::ValueAccessor;

pub fn get_cascade_conditions(cascades: Option<ValueAccessor>) -> Vec<String> {
    if let Some(cascades) = cascades {
        cascades
            .list()
            .unwrap()
            .iter()
            .map(|field| field.string().unwrap().to_string())
            .collect::<Vec<String>>()
    } else {
        Vec::new()
    }
}