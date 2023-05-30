// Compile vs Runtime reading from environment variable and file

// Ex:
// https://github.com/simsekgokhan/zapatos/commit/ae4ff6e3069a94a9775885246c92d05204d05b68?diff=unified

// 1. Read file bytes at Compile time - this will error at build time if MRB_PATH
//    is not defined
//
// #[cfg(unix)]
// const RLS_BUNDLE_BYTES: &[u8] = include_bytes!(concat!(env!("MRB_PATH"), "/head.mrb"));
// #[cfg(windows)]
// const RLS_BUNDLE_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "\\head.mrb"));

// static HEAD_RELEASE_BUNDLE: Lazy<ReleaseBundle> = Lazy::new(|| {
//     bcs::from_bytes::<ReleaseBundle>(RLS_BUNDLE_BYTES).expect("bcs succeeds")
// });
// pub fn head_release_bundle() -> &'static ReleaseBundle {
//     &HEAD_RELEASE_BUNDLE
// }

// 2. Same as 1, but at runtime
#[test] #[should_panic] 
fn ex1() {
    let mrb_path = std::env::var("MRB_PATH").expect("failed to read env.var. MRB_PATH");
    let mrb_bytes = std::fs::read(mrb_path).expect("unable to read head.mrb file");
}