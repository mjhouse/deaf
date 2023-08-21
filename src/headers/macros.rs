
macro_rules! prop {
    ( $n:ident, $t:ty ) => {
        paste! {
            pub fn [< get_ $n >](&self) -> $t {
                get!(self,$n)
            }
        
            pub fn [< set_ $n >](&mut self, v: $t) -> Result<()> {
                set!(self,$n,v)
            }  
        }
    }
}

macro_rules! pass {
    ( $n:ident, $p:ident, $t:ty ) => {
        paste! {
            pub fn $n(&self) -> $t {
                self.0.[< get_ $p >]()
            }
        
            pub fn [< set_ $n >](&mut self, v: $t) -> Result<()> {
                self.0.[< set_ $p >](v)
            }  
        }
    }
}