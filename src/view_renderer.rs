use rocket::response::content;
use tera::{Context, Result, Tera};

lazy_static! {
    pub static ref VIEW_RENDERER: ViewRenderer = ViewRenderer::new();
}

pub struct ViewRenderer {
    tera: Tera,
}

impl ViewRenderer {
    fn new() -> Self {
        ViewRenderer {
            tera: Tera::new("views/**/*").unwrap(),
        }
    }

    pub fn render_html(&self, template_name: &str, ctx: Context) -> Result<content::RawHtml<String>> {
        self.tera.render(template_name, &ctx).map(content::RawHtml)
    }
}
