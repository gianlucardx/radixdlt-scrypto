use sbor::any::Value;
use scrypto::types::{Address, Bid, Decimal, Rid};

#[derive(Debug, Clone)]
pub struct Transaction {
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    DeclareTempBucket {
        name: String,
    },

    DeclareTempBucketRef {
        name: String,
    },

    TakeFromContext {
        amount: String,
        resource_address: String,
        to: String,
    },

    BorrowFromContext {
        amount: String,
        resource_address: String,
        to: String,
    },

    CallFunction {
        package_address: String,
        blueprint_name: String,
        function: String,
        args: Vec<Value>,
    },

    CallMethod {
        component_address: String,
        method: String,
        args: Vec<Value>,
    },

    DropAllBucketRefs,

    DepositAllBuckets {
        account: String,
    },
}

pub enum Value {
    Unit,
    Bool(bool),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    String(String),

    Struct(Fields),
    Enum(u8, Fields),

    Option(Box<Option<Value>>),
    Box(Box<Value>),
    Array(u8, Vec<Value>),
    Tuple(Vec<Value>),
    Result(Box<Result<Value, Value>>),

    Vec(u8, Vec<Value>),
    TreeSet(u8, Vec<Value>),
    TreeMap(u8, u8, Vec<(Value, Value)>),
    HashSet(u8, Vec<Value>),
    HashMap(u8, u8, Vec<(Value, Value)>),
}

// `struct = 'struct' '(' fields ')'`
pub struct Struct {
    fields: Fields,
}

pub enum Fields {
    Named(Values)
}
struct({values} | (values) | unit)