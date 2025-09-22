use tihu::api::Response;
use tihu::SharedString;

#[derive(thiserror::Error, Debug)]
pub enum ErrNo {
    #[error("用户未登录")]
    LoginRequired,
    #[error("{0}")]
    CommonError(SharedString),
    #[error("{0}")]
    Other(#[from] anyhow::Error),
    #[error("配置错误，{0}")]
    ConfigError(SharedString),
    #[error("任务超时，{0}")]
    Timeout(SharedString),
    #[error("没有可用的{0}服务")]
    NoService(SharedString),
    #[error("{0}服务忙")]
    ServiceBusy(SharedString),
    #[error("暂停服务")]
    ServicePaused,
    #[error("接口不存在")]
    NoSuchApi,
    #[error("序列化数据失败,{0}")]
    SerializeError(serde_json::Error),
    #[error("反序列化数据失败,{0}")]
    DeserializeError(serde_json::Error),
    #[error("调用远程接口失败,{0}")]
    ApiError(anyhow::Error),
    #[error("只支持UTF-8格式的数据")]
    Utf8Only,
    #[error("参数格式不正确")]
    ParamFormatError,
    #[error("参数无效,{0}")]
    ParamInvalid(SharedString),
    #[error("令牌无效")]
    TokenInvalid,
    #[error("没有权限")]
    NotAllowed,
    #[error("访问太频繁")]
    TooFrequent,
    #[error("文件上传请求格式不正确")]
    MultipartRequired,
    #[error("未定义的枚举值,{0}")]
    UndefinedEnumValue(SharedString),
    #[error("没有数据库连接")]
    NoDbClient,
    #[error("预编译sql失败,{0}")]
    PrepareStatementError(anyhow::Error),
    #[error("查询数据失败,{0}")]
    QueryError(anyhow::Error),
    #[error("数据操作失败,{0}")]
    ExecuteError(anyhow::Error),
    #[error("开启数据库事务失败,{0}")]
    OpenTransactionError(anyhow::Error),
    #[error("获取数据字段失败,{0}")]
    ExtractDataError(anyhow::Error),
    #[error("提交数据库事务失败,{0}")]
    CommitTransactionError(anyhow::Error),
    #[error("没有缓存连接")]
    NoCacheClient,
    #[error("缓存操作失败,{0}")]
    CacheOperationError(anyhow::Error),
}

impl ErrNo {
    pub fn code(&self) -> i32 {
        return match *self {
            ErrNo::LoginRequired => -1,
            ErrNo::CommonError(_) => -2,
            ErrNo::Other(_) => -3,
            ErrNo::ConfigError(_) => -4,
            ErrNo::Timeout(_) => -5,
            ErrNo::NoService(_) => -6,
            ErrNo::ServiceBusy(_) => -7,
            ErrNo::ServicePaused => -8,
            ErrNo::NoSuchApi => -9,
            ErrNo::SerializeError(_) => -10,
            ErrNo::DeserializeError(_) => -11,
            ErrNo::ApiError(_) => -12,
            ErrNo::Utf8Only => -13,
            ErrNo::ParamFormatError => -14,
            ErrNo::ParamInvalid(_) => -15,
            ErrNo::TokenInvalid => -16,
            ErrNo::NotAllowed => -17,
            ErrNo::TooFrequent => -18,
            ErrNo::MultipartRequired => -19,
            ErrNo::UndefinedEnumValue(_) => -20,
            ErrNo::NoDbClient => -21,
            ErrNo::PrepareStatementError(_) => -22,
            ErrNo::QueryError(_) => -23,
            ErrNo::ExecuteError(_) => -24,
            ErrNo::OpenTransactionError(_) => -25,
            ErrNo::ExtractDataError(_) => -26,
            ErrNo::CommitTransactionError(_) => -27,
            ErrNo::NoCacheClient => -28,
            ErrNo::CacheOperationError(_) => -29,
        };
    }
    pub fn message(&self) -> SharedString {
        return self.to_string().into();
    }
}

impl<T> From<ErrNo> for Response<T> {
    fn from(err_no: ErrNo) -> Response<T> {
        return Response::failure(err_no.code(), err_no.message(), None);
    }
}

pub fn open_transaction_error<E>(error: E) -> ErrNo
where
    E: std::error::Error + Send + Sync + 'static,
{
    ErrNo::OpenTransactionError(error.into())
}

pub fn prepare_statement_error<E>(error: E) -> ErrNo
where
    E: std::error::Error + Send + Sync + 'static,
{
    ErrNo::PrepareStatementError(error.into())
}

pub fn query_error<E>(error: E) -> ErrNo
where
    E: std::error::Error + Send + Sync + 'static,
{
    ErrNo::QueryError(error.into())
}

pub fn undefined_enum_value(err_msg: SharedString) -> ErrNo {
    ErrNo::UndefinedEnumValue(err_msg)
}

pub fn extract_data_error<E>(error: E) -> ErrNo
where
    E: std::error::Error + Send + Sync + 'static,
{
    ErrNo::ExtractDataError(error.into())
}

pub fn execute_error<E>(error: E) -> ErrNo
where
    E: std::error::Error + Send + Sync + 'static,
{
    ErrNo::ExecuteError(error.into())
}

pub fn commit_transaction_error<E>(error: E) -> ErrNo
where
    E: std::error::Error + Send + Sync + 'static,
{
    ErrNo::CommitTransactionError(error.into())
}
