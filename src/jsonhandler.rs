use std::collections::HashMap;

pub fn map_from_json(json_serial: String) -> HashMap::<String, String>{
    let mut result = HashMap::<String, String>::new();

    let iter : String = json_serial.chars().filter(|a| a.is_alphanumeric() || a.to_owned() == '_' || a.to_owned() == ',' || a.to_owned() == ':').collect();
    let pairs = iter.split(',');
    for pair in pairs{
        let mut a = pair.split(':');
        if a.clone().count() < 2 { continue};
        let first = a.next().unwrap();
        let last = a.next_back().unwrap();

        result.insert(first.to_string(), last.to_string());
    }
    
    return result;
}