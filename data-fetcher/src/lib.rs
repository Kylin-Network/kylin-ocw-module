#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    debug, decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult,
    storage::StorageMap,
};

use frame_system::{
    self as system, ensure_signed,
    offchain::{AppCrypto, CreateSignedTransaction, SendSignedTransaction, Signer},
};
use serde_json;
use sp_core::crypto::KeyTypeId;
use sp_runtime::offchain as rt_offchain;
use sp_std::{prelude::*, str};

pub trait Trait: system::Trait + CreateSignedTransaction<Call<Self>> {
    type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
    type Call: From<Call<Self>>;
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// Define default proxy endpoint and API name of data proxy
pub const DEFAULT_DATA_PROXY_ENDPOINT: &str = "http://localhost:8080";
pub const DEFAULT_DATA_PROXY_API_NAME: &str = "bitmex_large_order_list";
pub const FETCH_TIMEOUT_PERIOD: u64 = 10_000; // in milli-seconds
pub const LOCK_TIMEOUT_EXPIRATION: u64 = FETCH_TIMEOUT_PERIOD + 1000; // in milli-seconds
pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"dftc"); // Data Fetcher

pub mod crypto {
    use crate::KEY_TYPE;
    use sp_core::sr25519::Signature as Sr25519Signature;
    use sp_runtime::app_crypto::{app_crypto, sr25519};
    use sp_runtime::{traits::Verify, MultiSignature, MultiSigner};

    app_crypto!(sr25519, KEY_TYPE);

    pub struct DataFetcherAuthId;
    impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for DataFetcherAuthId {
        type RuntimeAppPublic = Public;
        type GenericSignature = sp_core::sr25519::Signature;
        type GenericPublic = sp_core::sr25519::Public;
    }

    impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
        for DataFetcherAuthId
    {
        type RuntimeAppPublic = Public;
        type GenericSignature = sp_core::sr25519::Signature;
        type GenericPublic = sp_core::sr25519::Public;
    }
}

decl_storage! {
    trait Store for Module<T: Trait> as DataFetcherModule {
        // `api_name` parmas will be invoked by pallet, the name defined will be passed to data proxy
        // while invoking data API. The default value should be configured correctly based on the
        // data proxy service.
         ApiName get(fn api_name): Vec<u8> = DEFAULT_DATA_PROXY_API_NAME.as_bytes().to_vec();

         // `data_proxy_url` defines the base of API service, which points to local data proxy
         // service by default.
         DataProxyUrl get(fn data_proxy_url): Vec<u8> = DEFAULT_DATA_PROXY_ENDPOINT.as_bytes().to_vec();

         // Result saves data fetched from data proxy API, which will be sent to chain with signed
         // transaction.
         Rslt get(fn rslt): map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, Vec<u8>);
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
    {
        ApiListUpdated(Vec<u8>),
        ApiNameUpdated(Vec<u8>),
        DataProxyUrlUpdated(Vec<u8>),
        DataUpdated(AccountId),
    }
);

decl_error! {
    pub enum Error for Module<T: Trait> {
        InternalError,
        NetworkError,
        ApiNameError,
        ResponseFormatError,
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        type Error = Error<T>;

        fn deposit_event() = default;

        fn offchain_worker(block_number: T::BlockNumber) {
            debug::info!("DataFetcher: Prepare to fetch data and send signed tx");
            let res = Self::fetch_data_n_send_signed_tx();
            if let Err(e) = res {
            debug::error!("Error: {:?}", e);
        }
        }

        #[weight = 10000]
        fn submit_data_proxy_api_resp(origin, api_rslt: Vec<u8>) -> DispatchResult {
            debug::info!("DataFetcher: Enter submit_data_proxy_api_resp");
            let who = ensure_signed(origin)?;
            debug::info!("DataFetcher: submit_data_proxy_api_resp: ({:?}, {:?})", api_rslt, who);

            let api_name = ApiName::get();
            Rslt::<T>::insert(&api_name, (who.clone(), api_rslt.clone()));
            Ok(())
        }

        #[weight = 10000]
        fn update_data_proxy_url(origin, data_proxy_url_vec: Vec<u8>) -> Result<(), Error<T>> {
            debug::debug!("DataFetcher: Prepare to update data proxy url to {:?}", data_proxy_url_vec);
            DataProxyUrl::put(data_proxy_url_vec);
            Self::deposit_event(RawEvent::DataProxyUrlUpdated(DataProxyUrl::get()));
            Ok(())
        }

        #[weight = 10000]
        fn update_api_name(origin, api_name: Vec<u8>) -> Result<(), Error<T>> {
            debug::debug!("DataFetcher: Prepare to update api name to {:?}", api_name);
            ApiName::put(api_name);
            Self::deposit_event(RawEvent::ApiNameUpdated(ApiName::get()));
            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    fn _send_req_to_data_proxy(
        path: &str,
        body_str: &str,
        method: &str,
    ) -> Result<Vec<u8>, Error<T>> {
        // Get latest data proxy URL
        let data_proxy_vec = DataProxyUrl::get();
        let data_proxy_url = [str::from_utf8(&data_proxy_vec).unwrap(), path].join("");

        // HTTP request related
        let timeout = sp_io::offchain::timestamp()
            .add(rt_offchain::Duration::from_millis(FETCH_TIMEOUT_PERIOD));

        let body_vec = vec![body_str];

        let pending = match method {
            "get" => rt_offchain::http::Request::get(&data_proxy_url)
                .add_header("Content-Type", "application/json")
                .deadline(timeout)
                .send(),
            "post" => rt_offchain::http::Request::post(&data_proxy_url, body_vec)
                .add_header("Content-Type", "application/json")
                .deadline(timeout)
                .send(),
            &_ => panic!("Wrong http method"),
        }
        .map_err(|e| {
            debug::error!("DataFetcher: pending req error: {:?}", e);
            <Error<T>>::NetworkError
        })?;

        debug::info!("DataFetcher: pending: {:?}", pending);

        let response = pending
            .try_wait(timeout)
            .map_err(|e| {
                debug::error!("DataFetcher: data proxy request error: {:?}", e);
                <Error<T>>::NetworkError
            })?
            .map_err(|e| {
                debug::error!("DataFetcher: data proxy request error: {:?}", e);
                <Error<T>>::NetworkError
            })?;

        debug::info!("DataFetcher: response: {:?}", response);

        if response.code != 200 {
            debug::error!("Unexpected http request status code: {}", response.code);
            return Err(<Error<T>>::NetworkError);
        }

        Ok(response.body().collect::<Vec<u8>>())
    }

    fn fetch_data_n_send_signed_tx() -> Result<(), Error<T>> {
        debug::info!("DataFetcher: Prepare to fetch api_list");
        let signer = Signer::<T, T::AuthorityId>::all_accounts();
        if !signer.can_sign() {
            debug::error!(
                "No local accounts available. Consider adding one via `author_insertKey` RPC."
            );
            return Err(<Error<T>>::InternalError);
        }

        let api_list_bytes = match Self::_send_req_to_data_proxy("/api_list", "", "get") {
            Ok(data) => {
                debug::info!("DataFetcher: api data received: {:?}", data);
                data
            }
            Err(err) => {
                debug::error!("DataFetcher: api data error: {:?}", err);
                return Err(err);
            }
        };

        let api_list_str = sp_std::str::from_utf8(&api_list_bytes).map_err(|_| {
            debug::info!("No UTF8 body");
            <Error<T>>::NetworkError
        })?;

        let api_list: Vec<&str> = match serde_json::from_str(api_list_str) {
            Ok(data) => data,
            Err(_) => {
                return Err(<Error<T>>::NetworkError);
            }
        };

        let given_api_name_bytes = ApiName::get().clone();
        let given_api_name = str::from_utf8(&given_api_name_bytes).unwrap();

        debug::info!(
            "DataFetcher: API list: {:?}, given_api_name: {:?}",
            api_list,
            given_api_name
        );

        if api_list.iter().any(|&s| s == given_api_name) {
            debug::info!("DataFetcher: API {:?} found", given_api_name);
        }

        if api_list.contains(&given_api_name) {
            debug::info!("Found API {} in data proxy", given_api_name);
            let body_str = ["{\"api_name\":\"", given_api_name, "\"}"].join("");
            let api_resp = Self::_send_req_to_data_proxy("/", &body_str, "post")
                .map_err(|e| {
                    debug::error!("fetch_proxy_data error: {:?}", e);
                    <Error<T>>::NetworkError
                })
                .unwrap();

            let tx_result = signer.send_signed_transaction(|_acct| {
                debug::info!(
                    "DataFetcher: Prepare to send sign tx, signer id: {:?}",
                    _acct.id
                );
                Call::submit_data_proxy_api_resp(api_resp.clone())
            });

            for (acc, res) in &tx_result {
                match res {
                    Ok(()) => {
                        debug::info!("[{:?}] sent API data: {:?}", acc.id, api_list_bytes)
                    }
                    Err(e) => {
                        debug::error!("[{:?}] Failed to submit transaction: {:?}", acc.id, e)
                    }
                }
            }
        } else {
            debug::error!(
                "API name specified {} is not existing in data proxy",
                given_api_name
            );
            return Err(<Error<T>>::ApiNameError);
        }

        Ok(())
    }
}

impl<T: Trait> rt_offchain::storage_lock::BlockNumberProvider for Module<T> {
    type BlockNumber = T::BlockNumber;
    fn current_block_number() -> Self::BlockNumber {
        <frame_system::Module<T>>::block_number()
    }
}
