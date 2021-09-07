/* automatically generated by rust-bindgen 0.59.1 */

pub const MAX_MESSAGE_ID_LENGTH: u32 = 256;
pub const MAX_TOPIC_LENGTH: u32 = 512;
pub const MAX_BROKER_NAME_ID_LENGTH: u32 = 256;
pub const MAX_SDK_VERSION_LENGTH: u32 = 256;
pub const DEFAULT_SDK_VERSION: &'static [u8; 15usize] = b"DefaultVersion\0";
pub const MAX_EXEPTION_MSG_LENGTH: u32 = 512;
pub const MAX_EXEPTION_FILE_LENGTH: u32 = 256;
pub const MAX_EXEPTION_TYPE_LENGTH: u32 = 128;
pub const _CStatus__OK: _CStatus_ = 0;
pub const _CStatus__NULL_POINTER: _CStatus_ = 1;
pub const _CStatus__MALLOC_FAILED: _CStatus_ = 2;
pub const _CStatus__PRODUCER_ERROR_CODE_START: _CStatus_ = 10;
pub const _CStatus__PRODUCER_START_FAILED: _CStatus_ = 10;
pub const _CStatus__PRODUCER_SEND_SYNC_FAILED: _CStatus_ = 11;
pub const _CStatus__PRODUCER_SEND_ONEWAY_FAILED: _CStatus_ = 12;
pub const _CStatus__PRODUCER_SEND_ORDERLY_FAILED: _CStatus_ = 13;
pub const _CStatus__PRODUCER_SEND_ASYNC_FAILED: _CStatus_ = 14;
pub const _CStatus__PRODUCER_SEND_ORDERLYASYNC_FAILED: _CStatus_ = 15;
pub const _CStatus__PRODUCER_SEND_TRANSACTION_FAILED: _CStatus_ = 16;
pub const _CStatus__PUSHCONSUMER_ERROR_CODE_START: _CStatus_ = 20;
pub const _CStatus__PUSHCONSUMER_START_FAILED: _CStatus_ = 20;
pub const _CStatus__PULLCONSUMER_ERROR_CODE_START: _CStatus_ = 30;
pub const _CStatus__PULLCONSUMER_START_FAILED: _CStatus_ = 30;
pub const _CStatus__PULLCONSUMER_FETCH_MQ_FAILED: _CStatus_ = 31;
pub const _CStatus__PULLCONSUMER_FETCH_MESSAGE_FAILED: _CStatus_ = 32;
pub const _CStatus__Not_Support: _CStatus_ = 500;
pub const _CStatus__NOT_SUPPORT_NOW: _CStatus_ = -1;
pub type _CStatus_ = ::std::os::raw::c_int;
pub use self::_CStatus_ as CStatus;
pub const _CLogLevel__E_LOG_LEVEL_FATAL: _CLogLevel_ = 1;
pub const _CLogLevel__E_LOG_LEVEL_ERROR: _CLogLevel_ = 2;
pub const _CLogLevel__E_LOG_LEVEL_WARN: _CLogLevel_ = 3;
pub const _CLogLevel__E_LOG_LEVEL_INFO: _CLogLevel_ = 4;
pub const _CLogLevel__E_LOG_LEVEL_DEBUG: _CLogLevel_ = 5;
pub const _CLogLevel__E_LOG_LEVEL_TRACE: _CLogLevel_ = 6;
pub const _CLogLevel__E_LOG_LEVEL_LEVEL_NUM: _CLogLevel_ = 7;
pub type _CLogLevel_ = ::std::os::raw::c_uint;
pub use self::_CLogLevel_ as CLogLevel;
pub const _CMessageModel__BROADCASTING: _CMessageModel_ = 0;
pub const _CMessageModel__CLUSTERING: _CMessageModel_ = 1;
pub type _CMessageModel_ = ::std::os::raw::c_uint;
pub use self::_CMessageModel_ as CMessageModel;
pub const _CTraceModel__OPEN: _CTraceModel_ = 0;
pub const _CTraceModel__CLOSE: _CTraceModel_ = 1;
pub type _CTraceModel_ = ::std::os::raw::c_uint;
pub use self::_CTraceModel_ as CTraceModel;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _CMQException_ {
    pub error: ::std::os::raw::c_int,
    pub line: ::std::os::raw::c_int,
    pub file: [::std::os::raw::c_char; 256usize],
    pub msg: [::std::os::raw::c_char; 512usize],
    pub type_: [::std::os::raw::c_char; 128usize],
}
#[test]
fn bindgen_test_layout__CMQException_() {
    assert_eq!(
        ::std::mem::size_of::<_CMQException_>(),
        904usize,
        concat!("Size of: ", stringify!(_CMQException_))
    );
    assert_eq!(
        ::std::mem::align_of::<_CMQException_>(),
        4usize,
        concat!("Alignment of ", stringify!(_CMQException_))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CMQException_>())).error as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_CMQException_),
            "::",
            stringify!(error)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CMQException_>())).line as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_CMQException_),
            "::",
            stringify!(line)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CMQException_>())).file as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_CMQException_),
            "::",
            stringify!(file)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CMQException_>())).msg as *const _ as usize },
        264usize,
        concat!(
            "Offset of field: ",
            stringify!(_CMQException_),
            "::",
            stringify!(msg)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CMQException_>())).type_ as *const _ as usize },
        776usize,
        concat!(
            "Offset of field: ",
            stringify!(_CMQException_),
            "::",
            stringify!(type_)
        )
    );
}
pub type CMQException = _CMQException_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMessage {
    _unused: [u8; 0],
}
extern "C" {
    pub fn CreateMessage(topic: *const ::std::os::raw::c_char) -> *mut CMessage;
}
extern "C" {
    pub fn DestroyMessage(msg: *mut CMessage) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SetMessageTopic(
        msg: *mut CMessage,
        topic: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SetMessageTags(
        msg: *mut CMessage,
        tags: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SetMessageKeys(
        msg: *mut CMessage,
        keys: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SetMessageBody(
        msg: *mut CMessage,
        body: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SetByteMessageBody(
        msg: *mut CMessage,
        body: *const ::std::os::raw::c_char,
        len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SetMessageProperty(
        msg: *mut CMessage,
        key: *const ::std::os::raw::c_char,
        value: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SetDelayTimeLevel(
        msg: *mut CMessage,
        level: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetOriginMessageTopic(msg: *mut CMessage) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn GetOriginMessageTags(msg: *mut CMessage) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn GetOriginMessageKeys(msg: *mut CMessage) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn GetOriginMessageBody(msg: *mut CMessage) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn GetOriginMessageProperty(
        msg: *mut CMessage,
        key: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn GetOriginDelayTimeLevel(msg: *mut CMessage) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMessageExt {
    _unused: [u8; 0],
}
extern "C" {
    pub fn GetMessageTopic(msgExt: *mut CMessageExt) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn GetMessageTags(msgExt: *mut CMessageExt) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn GetMessageKeys(msgExt: *mut CMessageExt) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn GetMessageBody(msgExt: *mut CMessageExt) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn GetMessageProperty(
        msgExt: *mut CMessageExt,
        key: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn GetMessageId(msgExt: *mut CMessageExt) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn GetMessageDelayTimeLevel(msgExt: *mut CMessageExt) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetMessageQueueId(msgExt: *mut CMessageExt) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetMessageReconsumeTimes(msgExt: *mut CMessageExt) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetMessageStoreSize(msgExt: *mut CMessageExt) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetMessageBornTimestamp(msgExt: *mut CMessageExt) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn GetMessageStoreTimestamp(msgExt: *mut CMessageExt) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn GetMessageQueueOffset(msgExt: *mut CMessageExt) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn GetMessageCommitLogOffset(msgExt: *mut CMessageExt) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn GetMessagePreparedTransactionOffset(
        msgExt: *mut CMessageExt,
    ) -> ::std::os::raw::c_longlong;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CBatchMessage {
    _unused: [u8; 0],
}
extern "C" {
    pub fn CreateBatchMessage() -> *mut CBatchMessage;
}
extern "C" {
    pub fn AddMessage(batchMsg: *mut CBatchMessage, msg: *mut CMessage) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn DestroyBatchMessage(batchMsg: *mut CBatchMessage) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _CMessageQueue_ {
    pub topic: [::std::os::raw::c_char; 512usize],
    pub brokerName: [::std::os::raw::c_char; 256usize],
    pub queueId: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout__CMessageQueue_() {
    assert_eq!(
        ::std::mem::size_of::<_CMessageQueue_>(),
        772usize,
        concat!("Size of: ", stringify!(_CMessageQueue_))
    );
    assert_eq!(
        ::std::mem::align_of::<_CMessageQueue_>(),
        4usize,
        concat!("Alignment of ", stringify!(_CMessageQueue_))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CMessageQueue_>())).topic as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_CMessageQueue_),
            "::",
            stringify!(topic)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CMessageQueue_>())).brokerName as *const _ as usize },
        512usize,
        concat!(
            "Offset of field: ",
            stringify!(_CMessageQueue_),
            "::",
            stringify!(brokerName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CMessageQueue_>())).queueId as *const _ as usize },
        768usize,
        concat!(
            "Offset of field: ",
            stringify!(_CMessageQueue_),
            "::",
            stringify!(queueId)
        )
    );
}
pub type CMessageQueue = _CMessageQueue_;
extern "C" {
    pub fn GetLatestErrorMessage() -> *const ::std::os::raw::c_char;
}
pub const E_CTransactionStatus_E_COMMIT_TRANSACTION: E_CTransactionStatus = 0;
pub const E_CTransactionStatus_E_ROLLBACK_TRANSACTION: E_CTransactionStatus = 1;
pub const E_CTransactionStatus_E_UNKNOWN_TRANSACTION: E_CTransactionStatus = 2;
pub type E_CTransactionStatus = ::std::os::raw::c_uint;
pub use self::E_CTransactionStatus as CTransactionStatus;
pub const E_CSendStatus__E_SEND_OK: E_CSendStatus_ = 0;
pub const E_CSendStatus__E_SEND_FLUSH_DISK_TIMEOUT: E_CSendStatus_ = 1;
pub const E_CSendStatus__E_SEND_FLUSH_SLAVE_TIMEOUT: E_CSendStatus_ = 2;
pub const E_CSendStatus__E_SEND_SLAVE_NOT_AVAILABLE: E_CSendStatus_ = 3;
pub type E_CSendStatus_ = ::std::os::raw::c_uint;
pub use self::E_CSendStatus_ as CSendStatus;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _SendResult_ {
    pub sendStatus: CSendStatus,
    pub msgId: [::std::os::raw::c_char; 256usize],
    pub offset: ::std::os::raw::c_longlong,
}
#[test]
fn bindgen_test_layout__SendResult_() {
    assert_eq!(
        ::std::mem::size_of::<_SendResult_>(),
        272usize,
        concat!("Size of: ", stringify!(_SendResult_))
    );
    assert_eq!(
        ::std::mem::align_of::<_SendResult_>(),
        8usize,
        concat!("Alignment of ", stringify!(_SendResult_))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_SendResult_>())).sendStatus as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_SendResult_),
            "::",
            stringify!(sendStatus)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_SendResult_>())).msgId as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_SendResult_),
            "::",
            stringify!(msgId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_SendResult_>())).offset as *const _ as usize },
        264usize,
        concat!(
            "Offset of field: ",
            stringify!(_SendResult_),
            "::",
            stringify!(offset)
        )
    );
}
pub type CSendResult = _SendResult_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CProducer {
    _unused: [u8; 0],
}
pub type QueueSelectorCallback = ::std::option::Option<
    unsafe extern "C" fn(
        size: ::std::os::raw::c_int,
        msg: *mut CMessage,
        arg: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
pub type CSendSuccessCallback = ::std::option::Option<unsafe extern "C" fn(result: CSendResult)>;
pub type CSendExceptionCallback = ::std::option::Option<unsafe extern "C" fn(e: CMQException)>;
pub type COnSendSuccessCallback = ::std::option::Option<
    unsafe extern "C" fn(
        result: CSendResult,
        msg: *mut CMessage,
        userData: *mut ::std::os::raw::c_void,
    ),
>;
pub type COnSendExceptionCallback = ::std::option::Option<
    unsafe extern "C" fn(
        e: CMQException,
        msg: *mut CMessage,
        userData: *mut ::std::os::raw::c_void,
    ),
>;
pub type CLocalTransactionCheckerCallback = ::std::option::Option<
    unsafe extern "C" fn(
        producer: *mut CProducer,
        msg: *mut CMessageExt,
        data: *mut ::std::os::raw::c_void,
    ) -> CTransactionStatus,
>;
pub type CLocalTransactionExecutorCallback = ::std::option::Option<
    unsafe extern "C" fn(
        producer: *mut CProducer,
        msg: *mut CMessage,
        data: *mut ::std::os::raw::c_void,
    ) -> CTransactionStatus,
>;
extern "C" {
    pub fn CreateProducer(groupId: *const ::std::os::raw::c_char) -> *mut CProducer;
}
extern "C" {
    pub fn CreateOrderlyProducer(groupId: *const ::std::os::raw::c_char) -> *mut CProducer;
}
extern "C" {
    pub fn CreateTransactionProducer(
        groupId: *const ::std::os::raw::c_char,
        callback: CLocalTransactionCheckerCallback,
        userData: *mut ::std::os::raw::c_void,
    ) -> *mut CProducer;
}
extern "C" {
    pub fn DestroyProducer(producer: *mut CProducer) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn StartProducer(producer: *mut CProducer) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ShutdownProducer(producer: *mut CProducer) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ShowProducerVersion(producer: *mut CProducer) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn SetProducerNameServerAddress(
        producer: *mut CProducer,
        namesrv: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SetProducerNameServerDomain(
        producer: *mut CProducer,
        domain: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SetProducerGroupName(
        producer: *mut CProducer,
        groupName: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SetProducerInstanceName(
        producer: *mut CProducer,
        instanceName: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SetProducerSessionCredentials(
        producer: *mut CProducer,
        accessKey: *const ::std::os::raw::c_char,
        secretKey: *const ::std::os::raw::c_char,
        onsChannel: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SetProducerLogPath(
        producer: *mut CProducer,
        logPath: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SetProducerLogFileNumAndSize(
        producer: *mut CProducer,
        fileNum: ::std::os::raw::c_int,
        fileSize: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SetProducerLogLevel(producer: *mut CProducer, level: CLogLevel)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SetProducerSendMsgTimeout(
        producer: *mut CProducer,
        timeout: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SetProducerCompressLevel(
        producer: *mut CProducer,
        level: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SetProducerMaxMessageSize(
        producer: *mut CProducer,
        size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SetProducerMessageTrace(
        consumer: *mut CProducer,
        openTrace: CTraceModel,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SendMessageSync(
        producer: *mut CProducer,
        msg: *mut CMessage,
        result: *mut CSendResult,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SendBatchMessage(
        producer: *mut CProducer,
        msg: *mut CBatchMessage,
        result: *mut CSendResult,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SendMessageAsync(
        producer: *mut CProducer,
        msg: *mut CMessage,
        cSendSuccessCallback: CSendSuccessCallback,
        cSendExceptionCallback: CSendExceptionCallback,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SendAsync(
        producer: *mut CProducer,
        msg: *mut CMessage,
        cSendSuccessCallback: COnSendSuccessCallback,
        cSendExceptionCallback: COnSendExceptionCallback,
        userData: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SendMessageOneway(producer: *mut CProducer, msg: *mut CMessage)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SendMessageOnewayOrderly(
        producer: *mut CProducer,
        msg: *mut CMessage,
        selector: QueueSelectorCallback,
        arg: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SendMessageOrderly(
        producer: *mut CProducer,
        msg: *mut CMessage,
        callback: QueueSelectorCallback,
        arg: *mut ::std::os::raw::c_void,
        autoRetryTimes: ::std::os::raw::c_int,
        result: *mut CSendResult,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SendMessageOrderlyAsync(
        producer: *mut CProducer,
        msg: *mut CMessage,
        callback: QueueSelectorCallback,
        arg: *mut ::std::os::raw::c_void,
        cSendSuccessCallback: CSendSuccessCallback,
        cSendExceptionCallback: CSendExceptionCallback,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SendMessageOrderlyByShardingKey(
        producer: *mut CProducer,
        msg: *mut CMessage,
        shardingKey: *const ::std::os::raw::c_char,
        result: *mut CSendResult,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SendMessageTransaction(
        producer: *mut CProducer,
        msg: *mut CMessage,
        callback: CLocalTransactionExecutorCallback,
        userData: *mut ::std::os::raw::c_void,
        result: *mut CSendResult,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CPushConsumer {
    _unused: [u8; 0],
}
pub const E_CConsumeStatus_E_CONSUME_SUCCESS: E_CConsumeStatus = 0;
pub const E_CConsumeStatus_E_RECONSUME_LATER: E_CConsumeStatus = 1;
pub type E_CConsumeStatus = ::std::os::raw::c_uint;
pub use self::E_CConsumeStatus as CConsumeStatus;
pub type MessageCallBack = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut CPushConsumer, arg2: *mut CMessageExt) -> ::std::os::raw::c_int,
>;
extern "C" {
    pub fn CreatePushConsumer(groupId: *const ::std::os::raw::c_char) -> *mut CPushConsumer;
}
extern "C" {
    pub fn DestroyPushConsumer(consumer: *mut CPushConsumer) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn StartPushConsumer(consumer: *mut CPushConsumer) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ShutdownPushConsumer(consumer: *mut CPushConsumer) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ShowPushConsumerVersion(consumer: *mut CPushConsumer) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn SetPushConsumerGroupID(
        consumer: *mut CPushConsumer,
        groupId: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetPushConsumerGroupID(consumer: *mut CPushConsumer) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn SetPushConsumerNameServerAddress(
        consumer: *mut CPushConsumer,
        namesrv: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SetPushConsumerNameServerDomain(
        consumer: *mut CPushConsumer,
        domain: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Subscribe(
        consumer: *mut CPushConsumer,
        topic: *const ::std::os::raw::c_char,
        expression: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn RegisterMessageCallbackOrderly(
        consumer: *mut CPushConsumer,
        pCallback: MessageCallBack,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn RegisterMessageCallback(
        consumer: *mut CPushConsumer,
        pCallback: MessageCallBack,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn UnregisterMessageCallbackOrderly(consumer: *mut CPushConsumer) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn UnregisterMessageCallback(consumer: *mut CPushConsumer) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SetPushConsumerThreadCount(
        consumer: *mut CPushConsumer,
        threadCount: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SetPushConsumerMessageBatchMaxSize(
        consumer: *mut CPushConsumer,
        batchSize: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SetPushConsumerInstanceName(
        consumer: *mut CPushConsumer,
        instanceName: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SetPushConsumerSessionCredentials(
        consumer: *mut CPushConsumer,
        accessKey: *const ::std::os::raw::c_char,
        secretKey: *const ::std::os::raw::c_char,
        channel: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SetPushConsumerLogPath(
        consumer: *mut CPushConsumer,
        logPath: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SetPushConsumerLogFileNumAndSize(
        consumer: *mut CPushConsumer,
        fileNum: ::std::os::raw::c_int,
        fileSize: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SetPushConsumerLogLevel(
        consumer: *mut CPushConsumer,
        level: CLogLevel,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SetPushConsumerMessageModel(
        consumer: *mut CPushConsumer,
        messageModel: CMessageModel,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SetPushConsumerMaxCacheMessageSize(
        consumer: *mut CPushConsumer,
        maxCacheSize: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SetPushConsumerMaxCacheMessageSizeInMb(
        consumer: *mut CPushConsumer,
        maxCacheSizeInMb: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SetPushConsumerMessageTrace(
        consumer: *mut CPushConsumer,
        openTrace: CTraceModel,
    ) -> ::std::os::raw::c_int;
}
pub const E_CPullStatus_E_FOUND: E_CPullStatus = 0;
pub const E_CPullStatus_E_NO_NEW_MSG: E_CPullStatus = 1;
pub const E_CPullStatus_E_NO_MATCHED_MSG: E_CPullStatus = 2;
pub const E_CPullStatus_E_OFFSET_ILLEGAL: E_CPullStatus = 3;
pub const E_CPullStatus_E_BROKER_TIMEOUT: E_CPullStatus = 4;
pub type E_CPullStatus = ::std::os::raw::c_uint;
pub use self::E_CPullStatus as CPullStatus;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _CPullResult_ {
    pub pullStatus: CPullStatus,
    pub nextBeginOffset: ::std::os::raw::c_longlong,
    pub minOffset: ::std::os::raw::c_longlong,
    pub maxOffset: ::std::os::raw::c_longlong,
    pub msgFoundList: *mut *mut CMessageExt,
    pub size: ::std::os::raw::c_int,
    pub pData: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout__CPullResult_() {
    assert_eq!(
        ::std::mem::size_of::<_CPullResult_>(),
        56usize,
        concat!("Size of: ", stringify!(_CPullResult_))
    );
    assert_eq!(
        ::std::mem::align_of::<_CPullResult_>(),
        8usize,
        concat!("Alignment of ", stringify!(_CPullResult_))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CPullResult_>())).pullStatus as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_CPullResult_),
            "::",
            stringify!(pullStatus)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CPullResult_>())).nextBeginOffset as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_CPullResult_),
            "::",
            stringify!(nextBeginOffset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CPullResult_>())).minOffset as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_CPullResult_),
            "::",
            stringify!(minOffset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CPullResult_>())).maxOffset as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_CPullResult_),
            "::",
            stringify!(maxOffset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CPullResult_>())).msgFoundList as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_CPullResult_),
            "::",
            stringify!(msgFoundList)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CPullResult_>())).size as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_CPullResult_),
            "::",
            stringify!(size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CPullResult_>())).pData as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_CPullResult_),
            "::",
            stringify!(pData)
        )
    );
}
pub type CPullResult = _CPullResult_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CPullConsumer {
    _unused: [u8; 0],
}
extern "C" {
    pub fn CreatePullConsumer(groupId: *const ::std::os::raw::c_char) -> *mut CPullConsumer;
}
extern "C" {
    pub fn DestroyPullConsumer(consumer: *mut CPullConsumer) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn StartPullConsumer(consumer: *mut CPullConsumer) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ShutdownPullConsumer(consumer: *mut CPullConsumer) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ShowPullConsumerVersion(consumer: *mut CPullConsumer) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn SetPullConsumerGroupID(
        consumer: *mut CPullConsumer,
        groupId: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn GetPullConsumerGroupID(consumer: *mut CPullConsumer) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn SetPullConsumerNameServerAddress(
        consumer: *mut CPullConsumer,
        namesrv: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SetPullConsumerNameServerDomain(
        consumer: *mut CPullConsumer,
        domain: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SetPullConsumerSessionCredentials(
        consumer: *mut CPullConsumer,
        accessKey: *const ::std::os::raw::c_char,
        secretKey: *const ::std::os::raw::c_char,
        channel: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SetPullConsumerLogPath(
        consumer: *mut CPullConsumer,
        logPath: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SetPullConsumerLogFileNumAndSize(
        consumer: *mut CPullConsumer,
        fileNum: ::std::os::raw::c_int,
        fileSize: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SetPullConsumerLogLevel(
        consumer: *mut CPullConsumer,
        level: CLogLevel,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn FetchSubscriptionMessageQueues(
        consumer: *mut CPullConsumer,
        topic: *const ::std::os::raw::c_char,
        mqs: *mut *mut CMessageQueue,
        size: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ReleaseSubscriptionMessageQueue(mqs: *mut CMessageQueue) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Pull(
        consumer: *mut CPullConsumer,
        mq: *const CMessageQueue,
        subExpression: *const ::std::os::raw::c_char,
        offset: ::std::os::raw::c_longlong,
        maxNums: ::std::os::raw::c_int,
    ) -> CPullResult;
}
extern "C" {
    pub fn ReleasePullResult(pullResult: CPullResult) -> ::std::os::raw::c_int;
}