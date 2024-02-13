use leptos::*;

#[component]
pub fn QrCode(
    #[prop(into)] data: Signal<String>,
    #[prop(optional, into)] qr_image_size: Option<usize>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let qr_image_size = qr_image_size.unwrap_or(1024);
    let qr_data_url = move || {
        let png_bytes = qrcode_generator::to_png_to_vec_from_str(
            data.get(),
            qrcode_generator::QrCodeEcc::Medium,
            qr_image_size,
        )
        .expect("Failed to generate QR code");
        let png_base64 = base64::display::Base64Display::new(
            &png_bytes,
            &base64::engine::general_purpose::STANDARD,
        );
        format!("data:image/png;base64,{png_base64}")
    };

    view! {
        <img
            src=qr_data_url
            class=class
        />
    }
}
