use serde::Deserialize;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use tiny_http::{Header, Method, Request, Response, Server};

const PORT: u16 = 51234;
const MAX_BODY_BYTES: usize = 2 * 1024 * 1024;
const MAX_SLUG_LEN: usize = 50;

#[derive(Deserialize)]
struct CapturePayload {
    title: String,
    body: String,
    #[serde(default)]
    url: Option<String>,
    #[serde(default)]
    tags: Vec<String>,
}

pub fn spawn(notes_dir: PathBuf) {
    std::thread::spawn(move || {
        let addr = format!("127.0.0.1:{PORT}");
        let server = match Server::http(&addr) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("capture: failed to bind {addr}: {e}");
                return;
            }
        };
        eprintln!("capture: listening on http://{addr}");
        for mut request in server.incoming_requests() {
            let response = handle(&mut request, &notes_dir);
            let _ = request.respond(response);
        }
    });
}

fn handle(request: &mut Request, notes_dir: &Path) -> Response<std::io::Cursor<Vec<u8>>> {
    if request.method() == &Method::Options {
        return cors(empty(204));
    }
    if request.method() != &Method::Post || request.url() != "/capture" {
        return cors(json_error(404, "not found"));
    }

    let mut buf = String::new();
    if let Err(e) = request
        .as_reader()
        .take(MAX_BODY_BYTES as u64)
        .read_to_string(&mut buf)
    {
        return cors(json_error(400, &format!("read: {e}")));
    }

    let payload: CapturePayload = match serde_json::from_str(&buf) {
        Ok(p) => p,
        Err(e) => return cors(json_error(400, &format!("json: {e}"))),
    };

    if payload.title.trim().is_empty() {
        return cors(json_error(400, "title required"));
    }

    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let slug = slugify(&payload.title);
    let filename = if slug.is_empty() {
        format!("capture-{stamp}.md")
    } else {
        format!("capture-{stamp}-{slug}.md")
    };

    if let Err(e) = std::fs::create_dir_all(notes_dir) {
        return cors(json_error(500, &format!("mkdir: {e}")));
    }
    let path = notes_dir.join(&filename);
    let content = build_content(&payload);
    if let Err(e) = std::fs::write(&path, content) {
        return cors(json_error(500, &format!("write: {e}")));
    }

    let body = format!(
        r#"{{"path":"{}"}}"#,
        path.to_string_lossy().replace('\\', "\\\\").replace('"', "\\\"")
    );
    cors(json_response(201, &body))
}

fn build_content(p: &CapturePayload) -> String {
    let mut out = String::from("---\n");
    if !p.tags.is_empty() {
        out.push_str("tags: [");
        for (i, t) in p.tags.iter().enumerate() {
            if i > 0 {
                out.push_str(", ");
            }
            out.push_str(t);
        }
        out.push_str("]\n");
    }
    if let Some(url) = &p.url {
        out.push_str("source: ");
        out.push_str(url);
        out.push('\n');
    }
    out.push_str("---\n\n");
    out.push_str("# ");
    out.push_str(p.title.trim());
    out.push_str("\n\n");
    out.push_str(&p.body);
    if !p.body.ends_with('\n') {
        out.push('\n');
    }
    out
}

fn slugify(s: &str) -> String {
    let mut out = String::new();
    let mut prev_dash = true;
    for c in s.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c.to_ascii_lowercase());
            prev_dash = false;
        } else if !prev_dash {
            out.push('-');
            prev_dash = true;
        }
    }
    while out.ends_with('-') {
        out.pop();
    }
    if out.len() > MAX_SLUG_LEN {
        out.truncate(MAX_SLUG_LEN);
        while out.ends_with('-') {
            out.pop();
        }
    }
    out
}

fn empty(status: u32) -> Response<std::io::Cursor<Vec<u8>>> {
    Response::from_string("").with_status_code(status)
}

fn json_error(status: u32, msg: &str) -> Response<std::io::Cursor<Vec<u8>>> {
    let body = format!(r#"{{"error":"{}"}}"#, msg.replace('"', "\\\""));
    json_response(status, &body)
}

fn json_response(status: u32, body: &str) -> Response<std::io::Cursor<Vec<u8>>> {
    Response::from_string(body)
        .with_status_code(status)
        .with_header(
            Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap(),
        )
}

fn cors(resp: Response<std::io::Cursor<Vec<u8>>>) -> Response<std::io::Cursor<Vec<u8>>> {
    resp.with_header(
        Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap(),
    )
    .with_header(
        Header::from_bytes(
            &b"Access-Control-Allow-Methods"[..],
            &b"POST, OPTIONS"[..],
        )
        .unwrap(),
    )
    .with_header(
        Header::from_bytes(
            &b"Access-Control-Allow-Headers"[..],
            &b"Content-Type"[..],
        )
        .unwrap(),
    )
}
