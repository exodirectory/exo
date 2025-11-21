use quick_xml::events::Event;
use quick_xml::Reader;
use std::error::Error;

#[derive(Debug)]
pub struct Policy {
    pub name: String,
    pub explain_text: Option<String>,
}

pub fn parse_admx(path: &str) -> Result<Vec<Policy>, Box<dyn Error>> {
    let mut reader = Reader::from_file(path)?;
    reader.trim_text(true);
    let mut policies = Vec::new();
    let mut current_policy: Option<Policy> = None;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) if e.name().as_ref() == b"policy" => {
                let name = e.attributes()
                    .find(|a| a.as_ref().unwrap().key.as_ref() == b"name")
                    .and_then(|a| a.ok())
                    .and_then(|a| String::from_utf8(a.value.into_owned()).ok())
                    .unwrap_or_default();
                current_policy = Some(Policy { name, explain_text: None });
            }
            Event::End(e) if e.name().as_ref() == b"policy" => {
                if let Some(p) = current_policy.take() {
                    policies.push(p);
                }
            }
            _ => {}
        }
        buf.clear();
        if matches!(event, Event::Eof) { break; }
    }

    Ok(policies)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let policies = super::parse_admx("tests/sample.admx").unwrap();
        assert!(!policies.is_empty());
    }
}
