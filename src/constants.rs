pub const MNEMONIC_LEN: usize = 8;
pub const MAX_NAME_LEN: usize = 256;
pub const WELCOME_REQUEST_SIZE: usize = MAX_NAME_LEN + MNEMONIC_LEN + 1;
pub const WELCOME_RESPONSE_SIZE: usize = MNEMONIC_LEN;
pub const ENVIRONMENT_REQUEST_SIZE: usize = MAX_NAME_LEN + MNEMONIC_LEN + 1;
pub const ENVIRONMENT_RESPONSE_SIZE: usize = MNEMONIC_LEN + 82;

cfg_if::cfg_if! {
    if #[cfg(target_os="linux")] {
        pub const WORKING_DIR: &str = "/run/garnd";
        pub const WELCOME_SOCK_FILE_NAME: &str = "welcome.sock";
        pub const ABSTRACT_SOCK_NAME_PREFIX: &str = "garnd:";
        pub const WELCOME_SOCK_ABSTRACT_NAME: &str = "welcome";
        #[cfg(miri)] pub const PAGE_SIZE: usize = 4096;
    }
}