pub(crate) fn get_fur_directory() -> std::path::PathBuf {
    std::path::PathBuf::from(
        std::env::var("FUR_DIRECTORY").unwrap_or_else(|_| "/furdb".to_string()),
    )
}
