pub trait Curry<'a, T> {
    type Output;
    fn curry(self) -> Self::Output;
}
macro_rules! gen_impl {
	(OUT $l:lifetime $id:ident, $ret:ident)=>{
		Box<dyn FnOnce($id)->$ret + $l>
	};
	(OUT $l:lifetime $id:ident $($tail:ident)+,$ret:ident) => {
		Box<dyn FnOnce($id)->gen_impl!(OUT $l $($tail)+,$ret) + $l>
	};
	(FUNC $id:ident,$ret:ident {$meta:ident, $($para:ident)+})=>{
		Box::new(move |$id|{
			$meta($($para),+)
		})
	};
	(FUNC $id:ident $($tail:ident)+,$ret:ident {$meta:ident, $($para:ident)+})=>{
		Box::new(move |$id|{
           gen_impl!(FUNC $($tail)+, $ret {$meta, $($para)+})
		})
	};
}
macro_rules! gen_curry {
	($($t:ident)+) => {
		#[allow(non_snake_case)]
		impl<'a,$($t),+,F:'a,Ret>  Curry<'a,($($t),+,F,Ret)> for F
		where F:FnOnce($($t),+)->Ret,
		      $($t:'a),+
		{
			type Output = gen_impl!(OUT 'a $($t)+,Ret);
			fn curry(self)->Self::Output{
				gen_impl!(FUNC $($t)+, Ret {self, $($t)+})
			}
		}
	};
}

macro_rules! impl_curry{
	($($($id:ident)+),*)=>{
		$(gen_curry!($($id)+);)*
	};
}

impl_curry! {
    T0,
    T0 T1,
    T0 T1 T2,
    T0 T1 T2 T3,
    T0 T1 T2 T3 T4,
    T0 T1 T2 T3 T4 T5,
    T0 T1 T2 T3 T4 T5 T6,
    T0 T1 T2 T3 T4 T5 T6 T7,
    T0 T1 T2 T3 T4 T5 T6 T7 T8,
    T0 T1 T2 T3 T4 T5 T6 T7 T8 T9,
    T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10,
    T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11
}

#[cfg(test)]
mod test {
    use crate::Curry;
    #[test]
    fn test() {
        fn fun(_i: i32) -> i32 {
            0
        }
        let c1 = fun.curry();
        assert_eq!(c1(1), 0);
        fn fun2(_i: i32, _i2: f64) -> bool {
            false
        }
        let c1 = fun2.curry();
        assert_eq!(c1(1)(2.0), false);
        fn fun6(_i: (), _i2: (), _i3: (), _: (), _: (i32, u8), _: &str) {}
        let c1 = fun6.curry();
        assert_eq!(c1(())(())(())(())((1, 2))("hello"), ());
    }
}
