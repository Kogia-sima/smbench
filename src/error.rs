macro_rules! impl_error {
    (
        $(#[$outer:meta])*
        pub enum $name:ident {
            $($chunk:tt)*
        }
    ) => {
        impl_error!(
            @PARSE_INNER
            meta $(#[$outer])*
            name $name
            queue [ $($chunk)* ]
            items []
        );
    };
    (
        @PARSE_INNER
        meta $(#[$outer:meta])*
        name $name:ident
        queue []
        items [ $($item:tt)* ]
    ) => {
        impl_error!(
            @WRITE
            meta $(#[$outer])*
            name $name
            items [ $($item)* ]
        );
    };
    (
        @PARSE_INNER
        meta $(#[$outer:meta])*
        name $name:ident
        queue [ $var:ident { $desc:expr, $msg:expr, }, $($chunk:tt)* ]
        items [ $($item:tt)* ]
    ) => {
        impl_error!(
            @PARSE_INNER
            meta $(#[$outer])*
            name $name
            queue [ $($chunk)* ]
            items [
                $($item)*
                {
                    var $var
                    args []
                    args2 []
                    args3 []
                    desc ($desc)
                    msg ($msg)
                }
            ]
        );
    };
    (
        @PARSE_INNER
        meta $(#[$outer:meta])*
        name $name:ident
        queue [ $var:ident ($($arg:ident: $argtype:ty),*) { $desc:expr, $msg:expr, }, $($chunk:tt)* ]
        items [ $($item:tt)* ]
    ) => {
        impl_error!(
            @PARSE_INNER
            meta $(#[$outer])*
            name $name
            queue [ $($chunk)* ]
            items [
                $($item)*
                {
                    var $var
                    args [
                        $({
                            arg $arg
                            argtype $argtype
                        })*
                    ]
                    args2 [ ($($arg),*) ]
                    args3 [ ($($argtype),*) ]
                    desc ($desc)
                    msg ($msg)
                }
            ]
        );
    };
    (
        @WRITE
        meta $(#[$outer:meta])*
        name $name:ident
        items [
            $({
                var $var:ident
                args [
                    $({
                        arg $arg:ident
                        argtype $argtype:ty
                    })*
                ]
                args2 [ $($args2:tt)* ]
                args3 [ $($args3:tt)* ]
                desc ($desc:expr)
                msg ($msg:expr)
            })*
        ]
    ) => {
        $(#[$outer])*
        pub enum $name {
            $(
                #[doc=$desc]
                $var $($args3)*,
            )*
        }

        impl ::core::fmt::Display for $name {
            #[allow(unused_variables)]
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match self {
                    $(
                        Self::$var $($args2)* => write!(f, $desc),
                    )*
                }
            }
        }

        impl ::core::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match self {
                    $(
                        Self::$var $($args2)* => write!(f, $msg $(,$arg)*),
                    )*
                }
            }
        }

        impl ::std::error::Error for $name {}
    };
}

impl_error!(
    #[derive(Clone, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    pub enum Error {
        InvalidBenchmarkFunction(t: String) {
            "invalid benchmark function",
            "{}",
        },
    }
);
