use axum::response::Html;

use crate::app::app_response::AppResponse;

/// GET home page
/// /
pub async fn index() -> Result<Html<&'static str>, AppResponse> {
    Ok(Html(
        r#"<!DOCTYPE html>
<html>
  <head>
    <title>Welcome!</title>
    <style>html, body {
    font-family: Tahoma, Verdana, Arial, sans-serif;
    background: #1C1C1C;
    color: white;
    height: 100%;
    width: 100%;
    padding: 0;
    margin: 0;
}
.Root-Table {
    padding: 0;
    height: 100%;
    width: 100%;
    max-width: 700px;
    margin: 0 auto;
}
.Root-Table .Root-Td {
    padding: 30px;
}
.Box {
    background-color: #00000096;
    border: 1px solid #F65500;
    border-radius: 20px;
    padding: 40px;
    text-align: center;
}
.Box h1 {
    margin: 0;
}
.Box .Icon {
    width: 150px;
    height: 150px;
    display: flex;
    justify-content: center;
    justify-items: center;
    margin: 0 auto;
}
.Box .Icon img {
    width: 124px;
}</style>
  </head>
  <body>
    <table class="Root-Table">
      <tr class="Root-Tr">
        <td class="Root-Td">
          <div class="Box">
            <h1>Welcome!</h1>
            <div class="Icon">
              <img alt="Ferris" src="https://raw.gitcode.com/keygenqt_vz/web-api-benchmark/raw/main/rust-axum/assets/ferris.svg"/>
            </div>
            <div>REST API written in Kotlin &amp; Ktor.</div>
          </div>
        </td>
      </tr>
    </table>
  </body>
</html>"#,
    ))
}
