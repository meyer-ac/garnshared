pub const MNEMONIC_LEN: usize = 8;
pub const MAX_NAME_LEN: usize = 256;

cfg_if::cfg_if! {
    if #[cfg(target_os="linux")] {
        pub const WORKING_DIR: &str = "/run/garnd";
        pub const ABSTRACT_SOCK_NAME_PREFIX: &str = "garnd:";
        pub const WELCOME_SOCK_ABSTRACT_NAME: &str = "welcome";
        pub const SHM_FILE_NAME: &str = "shm";
    }
}

pub const DEFAULT_CACHE_LINE_PAIR_SIZE: usize = 128;