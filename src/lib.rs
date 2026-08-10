pub mod platform_traits;
pub mod error_types;

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        pub mod linux;
    }
}