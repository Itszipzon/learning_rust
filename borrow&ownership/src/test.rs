#[derive(Clone)]
pub struct CloneableTestStruct {
    pub field1: i32,
    pub field2: String,
}

impl CloneableTestStruct {
    pub fn new(field1: i32, field2: String) -> Self {
      CloneableTestStruct { field1, field2 }
    }

    pub fn display(&self) {
        println!("field1: {}, field2: {}", self.field1, self.field2);
    }
}

#[derive(Copy, Clone)]
pub struct CopyableTestStruct<'a> {
    pub field1: i32,
    pub field2: &'a str,
}

impl <'a> CopyableTestStruct<'a> {
    pub fn new(field1: i32, field2: &'a str) -> Self {
        CopyableTestStruct { field1, field2 }
    }

    pub fn display(&self) {
        println!("field1: {}, field2: {}", self.field1, self.field2);
    }
}