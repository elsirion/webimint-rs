mod db;

use fedimint_client::secret::PlainRootSecretStrategy;
use fedimint_core::db::mem_impl::MemDatabase;
use fedimint_core::task::TaskGroup;
use leptos::*;

pub fn main() {
    tracing_wasm::set_as_global_default();

    let (info_sender, info_receiver) = tokio::sync::mpsc::channel::<String>(1);
    let tg = TaskGroup::new();

    wasm_bindgen_futures::spawn_local(async move {
        info_sender.send(format!("Starting client")).await.unwrap();
        let mut client_builder = fedimint_client::Client::builder();
        client_builder.with_database(MemDatabase::new());
        client_builder.with_invite_code("fed11jpr3lgm8tuhcky2r3g287tgk9du7dd7kr95fptdsmkca7cwcvyu0lyqeh0e6rgp4u0shxsfaxycpwqpfwaehxw309askcurgvyhx6at5d9h8jmn9wsknqvfwv3jhvtnxv4jxjcn5vvhxxmmd9udpnpn49yg9w98dejw9u76hmm9".parse().expect("Invalid join code"));
        let client_res = client_builder.build::<PlainRootSecretStrategy>(tg).await;

        let client = match client_res {
            Ok(client) => client,
            Err(e) => {
                info_sender
                    .send(format!("Error initializing client: {e:?}"))
                    .await
                    .unwrap();
                return;
            }
        };

        let name = client
            .get_meta("federation_name")
            .unwrap_or("<unknown>".to_string());
        info_sender
            .send(format!("Client started! Connected to federation: {name}"))
            .await
            .unwrap();
    });

    let info_stream = tokio_stream::wrappers::ReceiverStream::new(info_receiver);

    console_error_panic_hook::set_once();
    mount_to_body(|cx| {
        let info_signal = create_signal_from_stream(cx, info_stream);
        view! { cx,
            <p>"Foo: " {info_signal}</p>
        }
    });
}
