pub fn nav_builder(hbs: &handlebars::Handlebars, auth: Option<String>) -> String {
    match auth {
        Some(_) => {
            let s = "<li><a href=\"/logout\">Logout</a></li>".to_owned();
            hbs.render_template(include_str!(r"./static/templates/nav.hbs"), &serde_json::json!({"logged_in": s})).unwrap()
        }
        None => {
            let s = "<li><a href=\"/register\">Register</a></li>\n<li><a href=\"/login\">Login</a></li>\n".to_owned();
            hbs.render_template(include_str!(r"./static/templates/nav.hbs"), &serde_json::json!({"logged_in": s})).unwrap()
        }
    }
}