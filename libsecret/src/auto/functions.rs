// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Schema;
use crate::SchemaType;
use glib::translate::*;

//pub fn attributes_build(schema: &Schema, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 } {
//    unsafe { TODO: call ffi:secret_attributes_build() }
//}

//pub fn attributes_buildv(schema: &Schema, va: /*Unknown conversion*//*Unimplemented*/Unsupported) -> /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 } {
//    unsafe { TODO: call ffi:secret_attributes_buildv() }
//}

pub fn get_schema(type_: SchemaType) -> Option<Schema> {
    assert_initialized_main_thread!();
    unsafe { from_glib_none(ffi::secret_get_schema(type_.to_glib())) }
}

//pub fn password_clear<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + Send + 'static>(schema: &Schema, cancellable: Option<&P>, callback: Q, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
//    unsafe { TODO: call ffi:secret_password_clear() }
//}

//
//pub fn password_clear_future(schema: &Schema, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {

//skip_assert_initialized!();
//let schema = schema.clone();
//Box_::pin(gio::GioFuture::new(&(), move |_obj, send| {
//    let cancellable = gio::Cancellable::new();
//    password_clear(
//        &schema,
//        Some(&cancellable),
//        ,
//        move |res| {
//            send.resolve(res);
//        },
//    );

//    cancellable
//}))
//}

//pub fn password_clear_sync<P: IsA<gio::Cancellable>>(schema: &Schema, cancellable: Option<&P>, error: &mut glib::Error, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> bool {
//    unsafe { TODO: call ffi:secret_password_clear_sync() }
//}

//pub fn password_clearv<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(schema: Option<&Schema>, attributes: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }, cancellable: Option<&P>, callback: Q) {
//    unsafe { TODO: call ffi:secret_password_clearv() }
//}

//pub fn password_clearv_sync<P: IsA<gio::Cancellable>>(schema: Option<&Schema>, attributes: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }, cancellable: Option<&P>) -> Result<(), glib::Error> {
//    unsafe { TODO: call ffi:secret_password_clearv_sync() }
//}

pub fn password_free(password: Option<&str>) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::secret_password_free(password.to_glib_none().0);
    }
}

//pub fn password_lookup<P: IsA<gio::Cancellable>, Q: FnOnce(Result<glib::GString, glib::Error>) + Send + 'static>(schema: &Schema, cancellable: Option<&P>, callback: Q, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
//    unsafe { TODO: call ffi:secret_password_lookup() }
//}

//
//pub fn password_lookup_future(schema: &Schema, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Pin<Box_<dyn std::future::Future<Output = Result<glib::GString, glib::Error>> + 'static>> {

//skip_assert_initialized!();
//let schema = schema.clone();
//Box_::pin(gio::GioFuture::new(&(), move |_obj, send| {
//    let cancellable = gio::Cancellable::new();
//    password_lookup(
//        &schema,
//        Some(&cancellable),
//        ,
//        move |res| {
//            send.resolve(res);
//        },
//    );

//    cancellable
//}))
//}

//pub fn password_lookup_binary_sync<P: IsA<gio::Cancellable>>(schema: &Schema, cancellable: Option<&P>, error: &mut glib::Error, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<Value> {
//    unsafe { TODO: call ffi:secret_password_lookup_binary_sync() }
//}

//pub fn password_lookup_nonpageable_sync<P: IsA<gio::Cancellable>>(schema: &Schema, cancellable: Option<&P>, error: &mut glib::Error, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<glib::GString> {
//    unsafe { TODO: call ffi:secret_password_lookup_nonpageable_sync() }
//}

//pub fn password_lookup_sync<P: IsA<gio::Cancellable>>(schema: &Schema, cancellable: Option<&P>, error: &mut glib::Error, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<glib::GString> {
//    unsafe { TODO: call ffi:secret_password_lookup_sync() }
//}

//pub fn password_lookupv<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(schema: Option<&Schema>, attributes: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }, cancellable: Option<&P>, callback: Q) {
//    unsafe { TODO: call ffi:secret_password_lookupv() }
//}

//pub fn password_lookupv_binary_sync<P: IsA<gio::Cancellable>>(schema: Option<&Schema>, attributes: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }, cancellable: Option<&P>) -> Result<Value, glib::Error> {
//    unsafe { TODO: call ffi:secret_password_lookupv_binary_sync() }
//}

//pub fn password_lookupv_nonpageable_sync<P: IsA<gio::Cancellable>>(schema: Option<&Schema>, attributes: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }, cancellable: Option<&P>) -> Result<glib::GString, glib::Error> {
//    unsafe { TODO: call ffi:secret_password_lookupv_nonpageable_sync() }
//}

//pub fn password_lookupv_sync<P: IsA<gio::Cancellable>>(schema: Option<&Schema>, attributes: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }, cancellable: Option<&P>) -> Result<glib::GString, glib::Error> {
//    unsafe { TODO: call ffi:secret_password_lookupv_sync() }
//}

//pub fn password_search<P: IsA<gio::Cancellable>, Q: FnOnce(Result<Vec<Retrievable>, glib::Error>) + Send + 'static>(schema: &Schema, flags: SearchFlags, cancellable: Option<&P>, callback: Q, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
//    unsafe { TODO: call ffi:secret_password_search() }
//}

//
//pub fn password_search_future(schema: &Schema, flags: SearchFlags, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Pin<Box_<dyn std::future::Future<Output = Result<Vec<Retrievable>, glib::Error>> + 'static>> {

//skip_assert_initialized!();
//let schema = schema.clone();
//Box_::pin(gio::GioFuture::new(&(), move |_obj, send| {
//    let cancellable = gio::Cancellable::new();
//    password_search(
//        &schema,
//        flags,
//        Some(&cancellable),
//        ,
//        move |res| {
//            send.resolve(res);
//        },
//    );

//    cancellable
//}))
//}

//pub fn password_search_sync<P: IsA<gio::Cancellable>>(schema: &Schema, flags: SearchFlags, cancellable: Option<&P>, error: &mut glib::Error, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Vec<Retrievable> {
//    unsafe { TODO: call ffi:secret_password_search_sync() }
//}

//pub fn password_searchv<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(schema: Option<&Schema>, attributes: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }, flags: SearchFlags, cancellable: Option<&P>, callback: Q) {
//    unsafe { TODO: call ffi:secret_password_searchv() }
//}

//pub fn password_searchv_sync<P: IsA<gio::Cancellable>>(schema: Option<&Schema>, attributes: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }, flags: SearchFlags, cancellable: Option<&P>) -> Result<Vec<Retrievable>, glib::Error> {
//    unsafe { TODO: call ffi:secret_password_searchv_sync() }
//}

//pub fn password_store<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + Send + 'static>(schema: &Schema, collection: Option<&str>, label: &str, password: &str, cancellable: Option<&P>, callback: Q, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
//    unsafe { TODO: call ffi:secret_password_store() }
//}

//
//pub fn password_store_future(schema: &Schema, collection: Option<&str>, label: &str, password: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {

//skip_assert_initialized!();
//let schema = schema.clone();
//let collection = collection.map(ToOwned::to_owned);
//let label = String::from(label);
//let password = String::from(password);
//Box_::pin(gio::GioFuture::new(&(), move |_obj, send| {
//    let cancellable = gio::Cancellable::new();
//    password_store(
//        &schema,
//        collection.as_ref().map(::std::borrow::Borrow::borrow),
//        &label,
//        &password,
//        Some(&cancellable),
//        ,
//        move |res| {
//            send.resolve(res);
//        },
//    );

//    cancellable
//}))
//}

//pub fn password_store_binary<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(schema: &Schema, collection: Option<&str>, label: &str, value: &Value, cancellable: Option<&P>, callback: Q, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
//    unsafe { TODO: call ffi:secret_password_store_binary() }
//}

//pub fn password_store_binary_sync<P: IsA<gio::Cancellable>>(schema: &Schema, collection: Option<&str>, label: &str, value: &Value, cancellable: Option<&P>, error: &mut glib::Error, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> bool {
//    unsafe { TODO: call ffi:secret_password_store_binary_sync() }
//}

//pub fn password_store_sync<P: IsA<gio::Cancellable>>(schema: &Schema, collection: Option<&str>, label: &str, password: &str, cancellable: Option<&P>, error: &mut glib::Error, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> bool {
//    unsafe { TODO: call ffi:secret_password_store_sync() }
//}

//pub fn password_storev<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(schema: Option<&Schema>, attributes: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }, collection: Option<&str>, label: &str, password: &str, cancellable: Option<&P>, callback: Q) {
//    unsafe { TODO: call ffi:secret_password_storev() }
//}

//pub fn password_storev_binary<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(schema: Option<&Schema>, attributes: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }, collection: Option<&str>, label: &str, value: &Value, cancellable: Option<&P>, callback: Q) {
//    unsafe { TODO: call ffi:secret_password_storev_binary() }
//}

//pub fn password_storev_binary_sync<P: IsA<gio::Cancellable>>(schema: Option<&Schema>, attributes: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }, collection: Option<&str>, label: &str, value: &Value, cancellable: Option<&P>) -> Result<(), glib::Error> {
//    unsafe { TODO: call ffi:secret_password_storev_binary_sync() }
//}

//pub fn password_storev_sync<P: IsA<gio::Cancellable>>(schema: Option<&Schema>, attributes: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }, collection: Option<&str>, label: &str, password: &str, cancellable: Option<&P>) -> Result<(), glib::Error> {
//    unsafe { TODO: call ffi:secret_password_storev_sync() }
//}

pub fn password_wipe(password: Option<&str>) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::secret_password_wipe(password.to_glib_none().0);
    }
}
