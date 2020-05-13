pub fn filter_comments(text: &str) -> String {
    let mut text = text;
    let mut state = false;
    let mut result = String::new();

    while !text.is_empty() {
        if !state {
            if let Some(idx) = text.find("<!--") {
                state = true;
                result.push_str(&text[..idx]);
                text = &text[idx..];
            } else {
                result.push_str(text);
                return result;
            }
        } else if let Some(idx) = text.find("-->") {
            state = false;
            text = &text[(idx + 3)..];
        }
    }

    result
}

pub fn filter_comments_copy_within(text: &mut String) {
    let mut idx = 0;
    let mut out = 0;
    loop {
        unsafe {
            let comment = match text.get_unchecked(idx..).find("<!--") {
                Some(offset) => offset,
                None => {
                    text.as_bytes_mut().copy_within(idx.., out);
                    out += text.len() - idx;
                    break;
                }
            };
            text.as_bytes_mut().copy_within(idx..idx + comment, out);
            out += comment;
            match text.get_unchecked(idx + comment + 4..).find("-->") {
                Some(len) => idx += comment + len + 7,
                None => break,
            }
        }
    }
    text.truncate(out);
}

pub fn filter_comments_custom_copy_within(text: String) -> String {
    use std::io::{Cursor, Seek, SeekFrom, Write};
    use std::{mem, ptr};

    fn unsafe_copy<'a, 'b>(text: &'a str) -> &'b str {
        let ptr = text.as_ptr();
        unsafe {
            let s = ptr::slice_from_raw_parts(ptr, text.len());
            mem::transmute(s)
        }
    }

    // We can skip all this nonsense if there are no comments.
    let split_idx = match text.find("<!--") {
        Some(idx) => idx,
        None => return text,
    };

    // Let the unholiness commence.
    let mut view = unsafe_copy(&text);
    let mut edit = Cursor::new(text.into_bytes());
    let mut inside_comment = true;
    let mut overall_length = split_idx + 1;

    edit.seek(SeekFrom::Start(split_idx as u64)).expect("wtf");
    view = &view[split_idx..];

    while !view.is_empty() {
        if inside_comment {
            if let Some(idx) = view.find("-->") {
                inside_comment = false;
                view = &view[(idx + 3)..];
            }
        } else {
            if let Some(idx) = view.find("<!--") {
                inside_comment = true;
                overall_length += idx;
                edit.write_all(&view[..idx].as_bytes()).unwrap();
                view = &view[idx..];
            } else {
                edit.write_all(view.as_bytes()).unwrap();
                let mut buffer = edit.into_inner();
                buffer.truncate(overall_length + view.len());
                unsafe {
                    return String::from_utf8_unchecked(buffer);
                }
            }
        }
    }

    let mut buffer = edit.into_inner();
    buffer.truncate(overall_length);
    return unsafe { String::from_utf8_unchecked(buffer) };
}
