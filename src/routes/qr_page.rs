use qrcode::render::svg;
use qrcode::QrCode;

pub fn qr_page(build: &str, address: &str) -> String {
    let install_url = format!("https://{}/install/{}", address, build);
    let code = QrCode::new(install_url.as_bytes()).unwrap();
    let image = code.render::<svg::Color>().build();

    format!(
        r#"
        <!DOCTYPE html>
        <html lang="es">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Código QR para {}</title>
            <style>
                {common_styles}
                .qr-container {{
                    text-align: center;
                }}
            </style>
        </head>
        <body>
            <div class="content-container">
                <h1>Código QR para {}</h1>
                <div class="qr-container">
                    {}
                    <p>Escanea este código QR para instalar {}.</p>
                    <p>URL de instalación: {}</p>
                </div>
            </div>
        </body>
        </html>
        "#,
        build, build, image, build, install_url,
        common_styles = include_str!("../common_styles.css")
    )
}