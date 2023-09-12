use leptos::*;
use web_sys::Storage;

pub fn local_storage() -> Storage {
  window().local_storage().unwrap().unwrap()
}