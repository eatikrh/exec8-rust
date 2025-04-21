
// src/parser.rs
pub struct Exec8Command {
    pub label: Option<String>,
    pub command: String,
    pub options: Vec<String>,
    pub parameters: Vec<String>,
    pub comment: Option<String>,
}

pub fn parse_control_line(line: &str) -> Exec8Command {
    let line = line.trim();

    let line = if let Some(stripped) = line.strip_prefix('@') {
        stripped.trim()
    } else {
        panic!("Control statement must start with '@'")
    };

    let (raw_stmt, comment) = if let Some(dot_pos) = line.find(" .") {
        let (stmt, comm) = line.split_at(dot_pos);
        (stmt.trim(), Some(comm[1..].trim().to_string()))
    } else {
        (line.trim(), None)
    };

    let comment = comment.map(|c| c.trim().to_string());

    let (label, body) = if let Some(pos) = raw_stmt.find(':') {
        (Some(raw_stmt[..pos].trim().to_string()), raw_stmt[pos + 1..].trim())
    } else {
        (None, raw_stmt)
    };

    let mut body_parts = body.split_whitespace();
    let command_with_options = body_parts.next().unwrap();
    let mut command_parts = command_with_options.split(',');
    let command = command_parts.next().unwrap().to_string();
    let options = command_parts.map(|s| s.trim().to_string()).collect::<Vec<_>>();

    let parameters = body_parts
        .flat_map(|s| s.split(','))
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();


    Exec8Command {
        label,
        command,
        options,
        parameters,
        comment,
    }
}
