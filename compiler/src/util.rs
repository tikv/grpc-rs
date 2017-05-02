/// Adjust method name to follow the rust's style.
pub fn to_snake_case(name: &str) -> String {
    let mut snake_method_name = String::with_capacity(name.len());
    let mut chars = name.chars();
    // initial char can be any char except '_'.
    let mut last_char = '.';
    'outer: while let Some(c) = chars.next() {
        // Please note that '_' is neither uppercase nor lowercase.
        if !c.is_uppercase() {
            last_char = c;
            snake_method_name.push(c);
            continue;
        }
        let mut can_append_underscore = false;
        // check if it's the first char.
        if !snake_method_name.is_empty() && last_char != '_' {
            snake_method_name.push('_');
        }
        last_char = c;
        // find all continous upper case char and append an underscore before
        // last upper case char if neccessary.
        while let Some(c) = chars.next() {
            if !c.is_uppercase() {
                if can_append_underscore && c != '_' {
                    snake_method_name.push('_');
                }
                snake_method_name.extend(last_char.to_lowercase());
                snake_method_name.push(c);
                last_char = c;
                continue 'outer;
            }
            snake_method_name.extend(last_char.to_lowercase());
            last_char = c;
            can_append_underscore = true;
        }
        snake_method_name.extend(last_char.to_lowercase());
    }
    snake_method_name
}

pub fn fq_grpc(item: &str) -> String {
    format!("::grpc::{}", item)
}

pub enum MethodType {
    Unary,
    ClientStreaming,
    ServerStreaming,
    Dulex,
}

#[cfg(test)]
mod test {
    #[test]
    fn test_snake_name() {
        let cases = vec![("AsyncRequest", "async_request"),
                         ("asyncRequest", "async_request"),
                         ("async_request", "async_request"),
                         ("createID", "create_id"),
                         ("CreateIDForReq", "create_id_for_req"),
                         ("Create_ID_For_Req", "create_id_for_req"),
                         ("ID", "id"),
                         ("id", "id")];

        for (origin, exp) in cases {
            let res = super::to_snake_case(&origin);
            assert_eq!(res, exp);
        }
    }
}
