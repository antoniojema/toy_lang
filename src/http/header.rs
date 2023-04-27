use crate::common::{
    macros::*,
    typedef::*
};

/******************/
/*  Header entry  */
/******************/
#[derive(Debug)]
pub struct HTTPHeaderEntry {
    pub key : String,
    pub value : String,
}

impl HTTPHeaderEntry {
    fn from_line(line : &str) -> Result<HTTPHeaderEntry, String> {
        let pos = unwrap_option_or_return!(
            line.find(':'),
            Err(String::from("Could not parse HTTP header entry."))
        );
        Ok(HTTPHeaderEntry{
            key: String::from((&line[..pos]).trim()),
            value: String::from((&line[pos+1..]).trim())
        })
    }

    pub fn to_string(&self) -> String {
        format!("{}: {}", self.key, self.value)
    }

    pub fn to_bytes(&self) -> Bytes {
        self.to_string().into_bytes()
    }
}

/************/
/*  Header  */
/************/
#[derive(Debug)]
pub struct HTTPHeader {
    pub entries : Vec<HTTPHeaderEntry>
}

impl HTTPHeader {
    pub fn empty() -> HTTPHeader {
        HTTPHeader { entries: vec![] }
    }

    pub fn to_string(&self) -> String {
        self.entries
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<String>>()
            .join("\r\n")
    }

    pub fn to_bytes(&self) -> Bytes {
        self.to_string().into_bytes()
    }

    pub fn find_key(&self, key : &str, any_case : bool) -> Option<usize> {
        let wanted_key = if any_case {key.to_uppercase()} else {String::from(key)};
        
        for (n, entry) in self.entries.iter().enumerate() {
            let entry_key = if any_case {entry.key.to_uppercase()} else {entry.key.clone()};
            
            if entry_key == *wanted_key {return Some(n);}
        }

        None
    }

    pub fn from(reader : &mut BufReader<TcpStream>) -> Result<HTTPHeader, String> {
        let mut entries = Vec::<HTTPHeaderEntry>::new();
        
        for line in reader.lines() {
            let line = unwrap_result_or_return!(
                line,
                Err(String::from("Error: Could not read stream line."))
            );
            if line.is_empty() {break;}

            let entry = unwrap_or_return!(
                HTTPHeaderEntry::from_line(&line)
            );

            entries.push(entry);
        }

        Ok(HTTPHeader{entries})
    }
}
