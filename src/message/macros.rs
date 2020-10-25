#[macro_use]
macro_rules! header {
    ($iter:expr, $header:path, $error:expr) => {
        $iter
            .find_map(|header| {
                if let $header(header) = header {
                    Some(header)
                } else {
                    None
                }
            })
            .ok_or($error)
    };
}

#[macro_use]
macro_rules! header_opt {
    ($iter:expr, $header:path) => {
        $iter.find_map(|header| {
            if let $header(header) = header {
                Some(header)
            } else {
                None
            }
        })
    };
}
