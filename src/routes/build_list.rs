use std::fs;

pub fn build_list() -> String {
    let mut content = String::new();
    
    match fs::read_dir("./builds") {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(file_name) = entry.file_name().to_str() {
                        if entry.file_type().unwrap().is_dir() {
                            content.push_str(&format!(
                                r#"<div class="item">
                                    <span class="folder">📁 {}</span>
                                    <div class="actions">
                                        <a href='/load_plist/{}' class='button install'>Instalar</a>
                                        <a href='/qr/{}' class='button qr'>Generar QR</a>
                                    </div>
                                </div>"#,
                                file_name, file_name, file_name
                            ));
                        }
                    }
                }
            }
        }
        Err(e) => {
            content = format!("<p class='error'>Error al leer el directorio: {}</p>", e);
        }
    }

    format!(
        r#"
        <!DOCTYPE html>
        <html lang="es">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Directorios disponibles</title>
            <style>
                {common_styles}
                .item {{
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    margin-bottom: 10px;
                    padding: 10px;
                    background-color: #f8f8f8;
                    border-radius: 4px;
                }}
                .folder {{ color: #FFA500; }}
                .actions {{ display: flex; gap: 10px; }}
                .button {{
                    padding: 5px 10px;
                    text-decoration: none;
                    color: white;
                    border-radius: 3px;
                }}
                .install {{ background-color: #4CAF50; }}
                .qr {{ background-color: #008CBA; }}
            </style>
        </head>
        <body>
            <div class="content-container">
                <h1>Directorios disponibles</h1>
                {content}
            </div>
        </body>
        </html>
        "#,
        common_styles = include_str!("../common_styles.css"),
        content = content
    )
}