pub fn clear() -> String {
    "\x1b[2J".to_string()
}

pub fn begin() -> String {
    "\x1bPq".to_string()
}

pub fn end() -> String {
    "\x1b\\".to_string()
}

#[cfg(test)]
mod tests {
}
