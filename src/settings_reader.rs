use std::collections::HashMap;


// Modify the settings in the extra_folder folder, nothing to see here.
pub fn get_settings() -> HashMap<String, String> {
    HashMap::<&str, &str>::from([
        ("window_width", "1600"),
        ("window_height", "1000"),
        ("window_title", "Pathfinding"),
        ("square_width", "50"),
        ("top_offset", "100")
    ]).into_iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
}
