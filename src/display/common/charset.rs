pub(crate) struct BaseCharset {
    cursor: char,
    block: char,
    empty: char,
}

impl BaseCharset {
    // Getter for `cursor`
    pub fn get_cursor(&self) -> char {
        self.cursor
    }

    // Getter for `block`
    pub fn get_block(&self) -> char {
        self.block
    }

    pub fn get_empty(&self) -> char {
        self.empty
    }

    // Setter for `cursor`
    pub fn set_cursor(&mut self, new_cursor: char) {
        self.cursor = new_cursor;
    }

    // Setter for `block`
    pub fn set_block(&mut self, new_block: char) {
        self.block = new_block;
    }

    pub fn set_empty(&mut self, new_empty: char) {
        self.empty = new_empty;
    }
}

pub(crate) fn get_base() -> BaseCharset {
    BaseCharset {
        cursor: '↪',
        block: '■',
        empty: ' ',
    }
}