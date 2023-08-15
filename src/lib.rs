use anyhow::{Ok, Result};
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

const HOME: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <title>"HTMX + Spin Example"</title>
</head>
<body>
    <button hx-post="/clicked" hx-swap="outerHTML">
        Click Me
    </button>

    <script src="https://unpkg.com/htmx.org@1.9.4"></script>
</body>

</html>
"#;

/// A simple Spin HTTP component.
#[http_component]
fn handle_htmx_example(req: Request) -> Result<Response> {
    println!("{:?}", req.headers());
    Ok(http::Response::builder()
        .status(200)
        .header("frontend", "htmx")
        .header("backend", "spin")
        .body(Some(HOME.into()))?)
}
