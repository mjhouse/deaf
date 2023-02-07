
/*
    Create a getter, setter and accessor:

    ```
    property!(class,ei_class,Width)
    ```

    expands to:

    ```
    // no-mut access to already-parsed value
    pub fn class(&self) -> Width;

    // re-parse a single value from the binary
    pub fn get_class(&self, b: &[u8]) -> Result<Width>;

    // set a new value in the binary
    pub fn set_class(&mut self, b: &mut [u8], v: Width) -> Result<()>;
    ```

*/
#[macro_export]
macro_rules! impl_property {
    ( $n: ident, $f: ident, $v: ident ) => {
        paste::paste!{
            pub fn $n(&self) -> $v {
                self.values.$f.clone()
            }
        
            pub fn [< get_ $n >](&self, b: &[u8]) -> Result<$v> {
                self.$f.get(b)
            }
        
            pub fn [< set_ $n >](&mut self, b: &mut [u8], v: $v) -> Result<()> {
                self.$f.set(b,v.clone())?;
                self.values.$f = v;
                Ok(())
            }
        }
    }
}

macro_rules! impl_constant {
    ( $f: ident, $t: ident, [ $( $n: tt => $m: ident ),+ ] ) => {

        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone, Copy, PartialEq, num_enum::IntoPrimitive, num_enum::TryFromPrimitive)]
        #[repr($f)]
        pub enum $t {
            $( $m ),+
        }

    }
}

macro_rules! impl_constant_nofail {
    ( $f: ident, $t: ident, [ $( $n: tt => $m: ident ),+ ] ) => {

        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone, Copy, PartialEq, num_enum::IntoPrimitive, num_enum::TryFromPrimitive)]
        #[repr($f)]
        pub enum $t {
            $( $m = $n ),+,
            #[num_enum(catch_all)]
            Unknown($f)
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

}