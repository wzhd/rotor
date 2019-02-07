mod internal;

use self::internal::GitGlobalConfigKey;

/// Set git config globally
pub fn global(key: &'static str) -> GitGlobalConfigKey {
    GitGlobalConfigKey { key }
}
