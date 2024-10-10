pub fn install_ipa(build: &str, address: &str) -> String {
    let plist_url = format!("https://{}/builds/{}", address, build);
    format!(
        r#"
        <!DOCTYPE html>
        <html lang="es">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Instalar {}</title>
            <style>
                {common_styles}
                .install-button {{ 
                    display: inline-block; 
                    padding: 10px 20px; 
                    background-color: #4CAF50; 
                    color: white; 
                    text-decoration: none; 
                    border-radius: 5px; 
                    font-size: 18px; 
                }}
            </style>
        </head>
        <body>
            <div class="content-container">
                <h1>Instalar {}</h1>
                <p>Haz clic en el botón de abajo para instalar la aplicación:</p>
                <a href="itms-services://?action=download-manifest&url={}" class="install-button">
                    Instalar {}
                </a>
            </div>
        </body>
        </html>
        "#,
        build,
        build,
        plist_url,
        build,
        common_styles = include_str!("../common_styles.css")
    )
}
