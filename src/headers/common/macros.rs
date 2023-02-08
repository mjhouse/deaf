
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

    $v:ident < $( $N:ident ),* >
*/
#[macro_export]
macro_rules! impl_property {
    // implements a property with the form: `impl_promperty!(NAME,FIELD,OUTPUT<GENERIC>)`
    ( $n: ident, $f: ident, $v:ident < $( $N:ident ),* > ) => {
        paste::paste!{
            pub fn $n(&self) -> $v< $( $N ),* > {
                self.values.$f.clone()
            }
        
            pub fn [< get_ $n >](&self, b: &[u8]) -> Result<$v< $( $N ),* >> {
                self.$f.get(b)
            }
        
            pub fn [< set_ $n >](&mut self, b: &mut [u8], v: $v< $( $N ),* >) -> Result<()> {
                self.$f.set(b,v.clone())?;
                self.values.$f = v;
                Ok(())
            }
        }
    };
    // implements a property with the form: `impl_promperty!(NAME,FIELD,OUTPUT)`
    ( $n: ident, $f: ident, $v: ident ) => {
        impl_property!($n,$f,$v<>);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}