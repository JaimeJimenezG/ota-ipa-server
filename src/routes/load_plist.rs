use html_escape::encode_text;
use std::{fs, path::PathBuf};

pub fn load_plist(build: &str, address: &str) -> String {
    let build_dir = format!("./builds/{}", build);

    let content = if let Ok(entries) = fs::read_dir(&build_dir) {
        let mut plist_content = String::new();
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.ends_with(".plist") {
                        let plist_path = entry.path();
                        match fs::read_to_string(&plist_path) {
                            Ok(content) => {
                                plist_content = content;
                                break;
                            }
                            Err(_) => {
                                return format!(
                                    "Error: No se pudo leer el archivo .plist en {}",
                                    build
                                )
                            }
                        }
                    }
                }
            }
        }
        if plist_content.is_empty() {
            format!(
                "Error: No se encontró ningún archivo .plist en el directorio {}",
                build
            )
        } else {
            plist_content
        }
    } else {
        format!("Error: No se pudo acceder al directorio {}", build)
    };

    let formatted_content = if content.starts_with("Error:") {
        format!("<p class='error'>{}</p>", content)
    } else {
        let escaped_content = encode_text(&content);
        format!("<pre><code>{}</code></pre>", escaped_content)
    };

    format!(
        r#"
        <!DOCTYPE html>
        <html lang="es">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Contenido del archivo .plist para {}</title>
            <style>
                {}
            </style>
        </head>
        <body>
            <div class="content-container">
                <h1>Contenido del archivo .plist para {}</h1>
                <div class="button-container">
                    <a href="itms-services://?action=download-manifest&amp;url={address}{url}" class="button install-button">install</a>
                </div>
                <div class="plist-content">
                    {}
                </div>
            </div>
        </body>
        </html>
        "#,
        build,
        include_str!("../common_styles.css"),
        build,
        formatted_content,
        url = &get_plist_path(build).unwrap().to_str().unwrap()[1..],
    )
}

fn get_plist_path(build: &str) -> Option<PathBuf> {
    let build_dir = PathBuf::from(format!("./builds/{}", build));

    if let Ok(entries) = fs::read_dir(&build_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.ends_with(".plist") {
                        return Some(entry.path());
                    }
                }
            }
        }
    }

    None
}
