

pub fn is_trigger(mess: &str) -> bool {
    let triggers: Vec<String> = vec![
        "nazi".to_string(),
        "hitler".to_string()
    ];

    let mess = mess.to_lowercase();

    for trigger in &triggers {
        if mess.contains(trigger) {
            return true;
        }
    }

    return false;
}