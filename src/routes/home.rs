pub fn home_page() -> String {
    format!(
        r#"
        <!DOCTYPE html>
        <html lang="es">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Servidor IPA - Página Principal</title>
            <style>
                {common_styles}
                .menu {{
                    list-style-type: none;
                    padding: 0;
                }}
                .menu li {{
                    margin-bottom: 10px;
                }}
                .menu a {{
                    display: block;
                    padding: 10px;
                    background-color: #f8f8f8;
                    border-radius: 4px;
                    text-decoration: none;
                    color: #333;
                }}
                .menu a:hover {{
                    background-color: #e8e8e8;
                }}
            </style>
        </head>
        <body>
            <div class="content-container">
                <h1>Bienvenido al Servidor IPA</h1>
                <ul class="menu">
                    <li><a href="/builds">Ver archivos builds disponibles</a></li>
                    <li><a href="/info">Información sobre cómo instalar .ipa</a></li>
                </ul>
            </div>
        </body>
        </html>
        "#,
        common_styles = include_str!("../common_styles.css")
    )
}