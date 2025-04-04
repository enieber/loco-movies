use loco_rs::prelude::*;

use crate::models::_entities::movies;

/// Render a list view of `movies`.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<movies::Model>) -> Result<Response> {
    format::render().view(v, "movies/list.html", data!({"items": items}))
}

/// Render a single `movies` view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &movies::Model) -> Result<Response> {
    format::render().view(v, "movies/show.html", data!({"item": item}))
}

/// Render a `movies` create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "movies/create.html", data!({}))
}

/// Render a `movies` edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &movies::Model) -> Result<Response> {
    format::render().view(v, "movies/edit.html", data!({"item": item}))
}
